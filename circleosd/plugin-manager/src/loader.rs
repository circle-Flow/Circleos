use anyhow::{Result, Context};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

use crate::manifest::{PluginManifest, PluginType};
use crate::sandbox::{WasmInstance};
use tracing::{info, warn};

#[derive(Debug)]
pub enum LoadedPlugin {
    Native { id: String, path: PathBuf, manifest: PluginManifest },
    Wasm { id: String, path: PathBuf, manifest: PluginManifest, instance: Arc<WasmInstance> },
}

#[derive(Clone)]
pub struct PluginManager {
    inner: Arc<RwLock<HashMap<String, LoadedPlugin>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn cleanup(&self) {
        // placeholder: cleanup unused wasm instances if needed
        // currently no-op
    }

    pub async fn list(&self) -> Vec<(String, String)> {
        let map = self.inner.read();
        map.iter().map(|(k, v)| {
            let t = match v {
                LoadedPlugin::Native { .. } => "native",
                LoadedPlugin::Wasm { .. } => "wasm",
            };
            (k.clone(), t.to_string())
        }).collect()
    }

    /// Load a plugin file (path) and optional manifest JSON path.
    /// If the manifest path is not provided, attempt to discover a JSON next to the file.
    pub async fn load_plugin(&self, path: &str, manifest_path: Option<&str>) -> Result<String> {
        let p = Path::new(path).to_path_buf();
        if !p.exists() {
            anyhow::bail!("plugin file not found: {}", path);
        }

        // load manifest
        let manifest = if let Some(mpath) = manifest_path {
            let txt = std::fs::read_to_string(mpath)?;
            serde_json::from_str::<PluginManifest>(&txt)?
        } else {
            // try file.json next to plugin file
            let mut guess = p.clone();
            guess.set_extension("json");
            if guess.exists() {
                let txt = std::fs::read_to_string(&guess)?;
                serde_json::from_str::<PluginManifest>(&txt)?
            } else {
                // fallback: infer type from extension
                let plugin_type = match p.extension().and_then(|s| s.to_str()).unwrap_or("") {
                    "wasm" => PluginType::Wasm,
                    _ => PluginType::Native,
                };
                PluginManifest {
                    name: p.file_stem().and_then(|s| s.to_str()).unwrap_or("plugin").to_string(),
                    version: None,
                    description: None,
                    plugin_type,
                    entry: None,
                }
            }
        };

        let id = Uuid::new_v4().to_string();
        match manifest.plugin_type {
            PluginType::Native => {
                // load native in-process using libloading (unsafe).
                // This is convenient for prototyping; for security, run native code in separate process.
                unsafe {
                    let lib = libloading::Library::new(&p)
                        .with_context(|| format!("failed to load library {}", p.display()))?;
                    // attempt to find init symbol, optional
                    if let Some(sym_name) = manifest.entry.as_ref() {
                        let symbol: libloading::Symbol<unsafe extern "C" fn()> =
                            lib.get(sym_name.as_bytes())
                            .with_context(|| format!("symbol {} not found in {}", sym_name, p.display()))?;
                        (symbol)();
                        // keep the library alive by leaking it into static memory (simple approach)
                        std::mem::forget(lib);
                    } else {
                        // if no entry point, keep lib alive anyway to maintain loaded symbols
                        std::mem::forget(lib);
                    }
                }
                let loaded = LoadedPlugin::Native { id: id.clone(), path: p, manifest };
                self.inner.write().insert(id.clone(), loaded);
                info!("loaded native plugin {}", id);
                Ok(id)
            }
            PluginType::Wasm => {
                // instantiate Wasm via wasmtime sandbox
                let instance = WasmInstance::new(&p).await?;
                let instance = Arc::new(instance);
                let loaded = LoadedPlugin::Wasm { id: id.clone(), path: p, manifest, instance: instance.clone() };
                self.inner.write().insert(id.clone(), loaded);
                info!("loaded wasm plugin {}", id);
                Ok(id)
            }
        }
    }

    pub async fn unload(&self, id: &str) -> Result<()> {
        let mut map = self.inner.write();
        if let Some(plugin) = map.remove(id) {
            match plugin {
                LoadedPlugin::Native { path, .. } => {
                    // native: cannot unload lib safely if leaked; in production use separate process model
                    warn!("native plugin unloaded (note: underlying library memory may remain allocated until process exit): {}", path.display());
                    Ok(())
                }
                LoadedPlugin::Wasm { instance, .. } => {
                    instance.shutdown().await?;
                    Ok(())
                }
            }
        } else {
            anyhow::bail!("plugin id not found");
        }
    }

    pub async fn invoke_wasm(&self, id: &str, func: &str, payload: &str) -> Result<String> {
        let map = self.inner.read();
        if let Some(plugin) = map.get(id) {
            if let LoadedPlugin::Wasm { instance, .. } = plugin {
                let resp = instance.call_func(func, payload).await?;
                Ok(resp)
            } else {
                anyhow::bail!("plugin is not wasm");
            }
        } else {
            anyhow::bail!("plugin id not found");
        }
    }
}
