use anyhow::Result;
use serde_json::json;
use crate::client;
use crate::config::CliConfig;

pub async fn run(action: &str, path_or_id: Option<String>) -> Result<()> {
    let cfg = CliConfig::load_or_default();
    match action {
        "list" => {
            let req = json!({"action":"list"}).to_string();
            let resp = client::send_unix_request(&cfg.plugin_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        "load" => {
            let path = path_or_id.ok_or_else(|| anyhow::anyhow!("path required"))?;
            let req = json!({"action":"load","path":path,"manifest":null}).to_string();
            let resp = client::send_unix_request(&cfg.plugin_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        "unload" => {
            let id = path_or_id.ok_or_else(|| anyhow::anyhow!("id required"))?;
            let req = json!({"action":"unload","id":id}).to_string();
            let resp = client::send_unix_request(&cfg.plugin_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        _ => {
            println!("unknown plugin action: {}", action);
        }
    }
    Ok(())
}
