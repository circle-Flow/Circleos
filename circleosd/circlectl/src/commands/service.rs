use anyhow::Result;
use serde_json::json;
use crate::client;
use crate::config::CliConfig;

pub async fn run(action: &str, name: Option<String>) -> Result<()> {
    let cfg = CliConfig::load_or_default();
    match action {
        "list" => {
            let req = json!({"action":"list"}).to_string();
            let resp = client::send_unix_request(&cfg.registry_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        "start" => {
            let svc = name.ok_or_else(|| anyhow::anyhow!("service name required"))?;
            let req = json!({"action":"start","name":svc}).to_string();
            let resp = client::send_unix_request(&cfg.registry_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        "stop" => {
            let svc = name.ok_or_else(|| anyhow::anyhow!("service name required"))?;
            let req = json!({"action":"stop","name":svc}).to_string();
            let resp = client::send_unix_request(&cfg.registry_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        _ => {
            println!("unknown service action: {}", action);
        }
    }
    Ok(())
}
