use anyhow::Result;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

pub fn init_tracing() -> Result<()> {
    // Use RUST_LOG env var or default to info
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = fmt::layer().with_target(false).json();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .ok();
    Ok(())
}
