use crate::security;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Default)]
pub struct PluginManager {
    plugins_dir: String,
    plugins: RwLock<HashMap<String, PluginInfo>>, // id -> info
}

impl PluginManager {
    pub fn new(plugins_dir: String) -> Self {
        PluginManager {
            plugins_dir,
            plugins: RwLock::new(HashMap::new()),
        }
    }

    pub async fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.read().await.values().cloned().collect()
    }

    /// Load an executable plugin by path (must be executable file).
    /// Verifies signature (stub) and then registers plugin.
    pub async fn load_plugin(&self, path: &str) -> Result<PluginInfo> {
        let pb = PathBuf::from(path);
        if !pb.exists() {
            return Err(anyhow::anyhow!("plugin path not found: {}", path));
        }

        // Attempt to verify signature (stub)
        let ok = security::verify_plugin_signature(path, None)?;
        if !ok {
            return Err(anyhow::anyhow!("plugin failed signature verification"));
        }

        // Register
        let id = Uuid::new_v4().to_string();
        let name = pb
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("plugin")
            .to_string();

        let info = PluginInfo {
            id: id.clone(),
            name,
            path: pb.to_string_lossy().to_string(),
        };

        self.plugins.write().await.insert(id.clone(), info.clone());

        Ok(info)
    }

    /// Example of invoking plugin with JSON-RPC over stdin/stdout.
    /// Sends a simple {"jsonrpc":"2.0","method":"process","params":{"input":...},"id":...}
    pub async fn call_plugin(&self, id: &str, input: &str) -> Result<serde_json::Value> {
        let plugins = self.plugins.read().await;
        let info = plugins.get(id).ok_or_else(|| anyhow::anyhow!("plugin not found"))?.clone();
        drop(plugins);

        let mut cmd = Command::new(&info.path);
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());

        let mut child = cmd.spawn()?;
        let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("no stdin"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("no stdout"))?;

        // Prepare request
        let req_id = Uuid::new_v4().to_string();
        let req = json!({
            "jsonrpc": "2.0",
            "method": "process",
            "params": {"input": input},
            "id": req_id
        });

        let mut writer = stdin;
        let req_text = serde_json::to_string(&req)? + "\n";
        writer.write_all(req_text.as_bytes()).await?;
        writer.flush().await?;
        drop(writer);

        // Read first line from stdout
        let mut reader = BufReader::new(stdout).lines();
        if let Some(line) = reader.next_line().await? {
            let v: serde_json::Value = serde_json::from_str(&line)?;
            // Try to wait for child
            let _ = child.wait().await;
            Ok(v)
        } else {
            let _ = child.wait().await;
            Err(anyhow::anyhow!("no response from plugin"))
        }
    }
}
