use anyhow::Result;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{info, debug, error};
use chrono::Local;
use std::fs;

mod service_registry;
mod auth;
mod plugin;
mod rpc;
mod config;

use service_registry::Registry;
use auth::AuthService;
use plugin::PluginManager;
use rpc::handle_rpc;

const LOG_PATH: &str = "var/log/circleosd.log";

fn log_to_file(msg: &str) {
    let now = Local::now().format("%H:%M:%S");
    let formatted = format!("[{now}] {msg}\n");
    let _ = fs::create_dir_all("var/log");
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .and_then(|mut f| std::io::Write::write_all(&mut f, formatted.as_bytes()));
    println!("{formatted}");
}

async fn boot_step(label: &str, delay_ms: u64, ok: bool) {
    log_to_file(&format!("[BOOT] {label}"));
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    if ok {
        log_to_file(&format!("[OK] {label}"));
    } else {
        log_to_file(&format!("[WARN] {label} (delayed)"));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    fs::create_dir_all("var/log")?;
    fs::create_dir_all("var/run")?;
    fs::create_dir_all("var/sessions")?;

    println!("====================================================");
    println!("🌀  CircleOSD Core Daemon Boot Sequence v0.2.0");
    println!("====================================================");

    boot_step("Powering on CPU cores...", 200, true).await;
    boot_step("Loading kernel services...", 250, true).await;
    boot_step("Initializing Service Registry...", 300, true).await;
    boot_step("Starting Auth Service...", 350, true).await;
    boot_step("Starting Plugin Manager...", 400, true).await;
    boot_step("Preparing RPC socket...", 150, true).await;

    // Load configuration
    let cfg = config::Config::load_default();

    // Shared components
    let registry = Arc::new(Registry::new());
    let auth = Arc::new(AuthService::new());
    let plugin_mgr = Arc::new(PluginManager::new());

    // Start background registry loop
    {
        let reg = registry.clone();
        tokio::spawn(async move {
            reg.run().await;
        });
    }

    // Prepare RPC socket
    let sock_path = &cfg.socket_path;
    let _ = std::fs::remove_file(sock_path);
    let listener = UnixListener::bind(sock_path)?;
    log_to_file(&format!("[SYSTEM] RPC socket ready on {sock_path}"));
    info!("Listening for RPC on {}", sock_path);

    log_to_file("[SYSTEM] Boot sequence completed.");
    println!("====================================================");
    println!("🟢 System ready for user login");
    println!("Use `circlectl user login` to authenticate.");
    println!("Logs → var/log/circleosd.log");
    println!("====================================================");

    // Accept RPC connections
    loop {
        let (stream, _) = listener.accept().await?;
        let registry = registry.clone();
        let auth = auth.clone();
        let plugin_mgr = plugin_mgr.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, registry, auth, plugin_mgr).await {
                error!("connection handling failed: {:?}", e);
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

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        debug!("got request: {}", line);
        let resp = handle_rpc(&line, registry.clone(), auth.clone(), plugin_mgr.clone()).await;
        let resp_text = serde_json::to_string(&resp)?;
        w.write_all(resp_text.as_bytes()).await?;
        w.write_all(b"\n").await?;
    }

    Ok(())
}
