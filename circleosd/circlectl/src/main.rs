use clap::{Parser, Subcommand};
use anyhow::Result;

mod client;
mod config;
mod commands;

#[derive(Parser)]
#[command(name = "circlectl")]
#[command(about = "CircleOSD command-line controller")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    User { action: String, username: Option<String> },
    Service { action: String, name: Option<String> },
    Plugin { action: String, path_or_id: Option<String> },
    System { action: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::User { action, username } => commands::user::run(&action, username).await?,
        Commands::Service { action, name } => commands::service::run(&action, name).await?,
        Commands::Plugin { action, path_or_id } => commands::plugin::run(&action, path_or_id).await?,
        Commands::System { action } => commands::system::run(&action).await?,
    }
    Ok(())
}
