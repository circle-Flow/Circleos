use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

mod registry;
mod service;
mod process;
mod health;

use registry::Registry;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logger
    tracing_subscriber::fmt::init();
    info!("service-registry starting...");

    let socket_path = PathBuf::from("/tmp/service-registry.sock");
    // remove stale socket
    let _ = std::fs::remove_file(&socket_path);

    let registry = Registry::new();

    // spawn registry background monitor (restarts, health checks)
    {
        let reg = registry.clone();
        tokio::spawn(async move {
            reg.run_monitor().await;
        });
    }

    // Start RPC listener
    registry.serve(socket_path).await?;

    Ok(())
}
