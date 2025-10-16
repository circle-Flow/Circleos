use anyhow::Result;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;

mod service_registry;
mod auth;
mod plugin;
mod rpc;
mod config;

use service_registry::Registry;
use auth::AuthService;
use plugin::PluginManager;
use rpc::{handle_rpc};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting circleosd core daemon...");

    // Load configuration
    let cfg = config::Config::load_default();

    // Shared components
    let registry = Arc::new(Registry::new());
    let auth = Arc::new(AuthService::new());
    let plugin_mgr = Arc::new(PluginManager::new());

    // Start registry background loop
    {
        let reg = registry.clone();
        tokio::spawn(async move {
            reg.run().await;
        });
    }

    // Ensure old socket removed then bind
    let sock_path = &cfg.socket_path;
    let _ = std::fs::remove_file(sock_path);
    let listener = UnixListener::bind(sock_path)?;
    info!("Listening for RPC on {}", sock_path);

    loop {
        let (stream, _) = listener.accept().await?;
        let registry = registry.clone();
        let auth = auth.clone();
        let plugin_mgr = plugin_mgr.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, registry, auth, plugin_mgr).await {
                tracing::error!("connection handling failed: {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: UnixStream,
    registry: Arc<Registry>,
    auth: Arc<AuthService>,
    plugin_mgr: Arc<PluginManager>,
) -> Result<()> {
    let (r, mut w) = stream.into_split();
    let mut reader = BufReader::new(r).lines();

    // Each line is a JSON-RPC request object
    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() { continue; }
        tracing::debug!("got request: {}", line);
        let resp = handle_rpc(&line, registry.clone(), auth.clone(), plugin_mgr.clone()).await;
        let resp_text = serde_json::to_string(&resp)?;
        w.write_all(resp_text.as_bytes()).await?;
        w.write_all(b"\n").await?;
    }

    Ok(())
}
