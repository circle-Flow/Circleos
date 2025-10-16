use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tracing::{info, error};

use crate::loader::PluginManager;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
enum Request {
    #[serde(rename = "list")]
    List {},

    #[serde(rename = "load")]
    Load { path: String, manifest: Option<String> },

    #[serde(rename = "unload")]
    Unload { id: String },

    #[serde(rename = "invoke")]
    Invoke { id: String, func: String, payload: String },
}

#[derive(Debug, Serialize)]
struct Response {
    ok: bool,
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

pub async fn serve(socket_path: PathBuf, manager: PluginManager) -> Result<()> {
    // remove old socket
    let _ = std::fs::remove_file(&socket_path);
    let listener = UnixListener::bind(&socket_path)?;
    // set socket permission to owner only
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&socket_path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&socket_path, perms)?;
    }
    info!("plugin-manager listening on {}", socket_path.display());

    loop {
        let (stream, _) = listener.accept().await?;
        let manager = manager.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, manager).await {
                error!("client error: {:?}", e);
            }
        });
    }
}

async fn handle_client(stream: UnixStream, manager: PluginManager) -> Result<()> {
    let (r, mut w) = stream.into_split();
    let mut reader = BufReader::new(r).lines();

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() { continue; }
        tracing::debug!("plugin-api <- {}", line);
        let req = serde_json::from_str::<Request>(&line);
        let resp = match req {
            Ok(Request::List {}) => {
                let items = manager.list().await;
                Response { ok: true, message: None, data: serde_json::to_value(items).ok() }
            }
            Ok(Request::Load { path, manifest }) => {
                match manager.load_plugin(&path, manifest.as_deref()).await {
                    Ok(id) => Response { ok: true, message: Some("loaded".into()), data: Some(serde_json::json!({ "id": id }))},
                    Err(e) => Response { ok: false, message: Some(format!("load failed: {}", e)), data: None },
                }
            }
            Ok(Request::Unload { id }) => {
                match manager.unload(&id).await {
                    Ok(_) => Response { ok: true, message: Some("unloaded".into()), data: None },
                    Err(e) => Response { ok: false, message: Some(format!("unload failed: {}", e)), data: None },
                }
            }
            Ok(Request::Invoke { id, func, payload }) => {
                match manager.invoke_wasm(&id, &func, &payload).await {
                    Ok(resp) => Response { ok: true, message: None, data: Some(serde_json::json!({ "resp": resp })) },
                    Err(e) => Response { ok: false, message: Some(format!("invoke failed: {}", e)), data: None },
                }
            }
            Err(e) => Response { ok: false, message: Some(format!("invalid request: {}", e)), data: None },
        };

        let out = serde_json::to_string(&resp)?;
        w.write_all(out.as_bytes()).await?;
        w.write_all(b"\n").await?;
    }

    Ok(())
}
