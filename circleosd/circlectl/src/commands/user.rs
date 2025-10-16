use anyhow::Result;
use serde_json::json;
use crate::client;
use crate::config::CliConfig;

pub async fn run(action: &str, username: Option<String>) -> Result<()> {
    let cfg = CliConfig::load_or_default();
    match action {
        "create" => {
            let name = username.ok_or_else(|| anyhow::anyhow!("username required"))?;
            // prompt for password (simple)
            let password = rpassword::prompt_password("Password: ")?;
            let req = json!({"action":"create_user","username":name,"password":password}).to_string();
            let resp = client::send_unix_request(&cfg.auth_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        "login" => {
            let name = username.ok_or_else(|| anyhow::anyhow!("username required"))?;
            let password = rpassword::prompt_password("Password: ")?;
            let req = json!({"action":"login","username":name,"password":password}).to_string();
            let resp = client::send_unix_request(&cfg.auth_socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        _ => {
            println!("unknown user action: {}", action);
        }
    }
    Ok(())
}
