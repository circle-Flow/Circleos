use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

mod api;
mod db;
mod hash;
mod session;

#[tokio::main]
async fn main() -> Result<()> {
    // init tracing logger
    tracing_subscriber::fmt::init();
    info!("Starting auth-service...");

    // default paths
    let db_path = PathBuf::from("./var/auth.db");
    let socket_path = PathBuf::from("/tmp/auth-service.sock");

    // ensure var directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // remove stale socket if present
    let _ = std::fs::remove_file(&socket_path);

    // initialize database
    let conn = db::init_db(&db_path)?;

    // start API listener (takes ownership of conn)
    api::serve(socket_path, conn).await?;

    Ok(())
}
