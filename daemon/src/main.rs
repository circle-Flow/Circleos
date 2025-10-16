mod api;
mod config;
mod plugin;
mod security;
mod service;
mod telemetry;

use crate::config::Config;
use crate::plugin::PluginManager;
use crate::service::ServiceManager;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing()?;

    info!("Bolt starting...");

    // Load config
    let config = Config::load_default().unwrap_or_default();
    info!("Loaded config: {:?}", config);

    // Initialize managers
    let svc_mgr = Arc::new(ServiceManager::new(config.services.clone()));
    let plugin_mgr = Arc::new(PluginManager::new(config.plugins_dir.clone()));

    // Start default services (non-blocking)
    svc_mgr.bootstrap().await;

    // Build HTTP server
    let app = api::router(svc_mgr.clone(), plugin_mgr.clone(), config.clone());

    let addr: SocketAddr = config.bind_addr.parse().unwrap_or_else(|_| "0.0.0.0:8080".parse().unwrap());
    info!("HTTP server starting at {}", addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    // graceful shutdown
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        tracing::error!("server error: {}", e);
    }

    info!("Bolt shutting down.");
    Ok(())
}

async fn shutdown_signal() {
    // Wait for ctrl+c or TERM
    let _ = signal::ctrl_c().await;
    tracing::info!("Shutdown signal received");
}
