use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

mod loader;
mod manifest;
mod sandbox;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("plugin-manager starting...");

    // default socket path (RPC)
    let socket_path = PathBuf::from("/tmp/plugin-manager.sock");
    // remove leftover socket if exists
    let _ = std::fs::remove_file(&socket_path);

    // instantiate manager
    let manager = loader::PluginManager::new();

    // spawn a small background task to auto-clean dead wasm instances (optional)
    let manager_clone = manager.clone();
    tokio::spawn(async move {
        loop {
            manager_clone.cleanup().await;
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    // start RPC server (simple newline JSON over unix socket)
    api::serve(socket_path, manager).await?;
    Ok(())
}
