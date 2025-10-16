use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service_registry::{Registry, Service};
use crate::auth::AuthService;
use crate::plugin::PluginManager;

/// Possible RPC requests (simple flattened protocol)
#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum RpcRequest {
    #[serde(rename = "create_user")]
    CreateUser { username: String, password: String },

    #[serde(rename = "auth")]
    Auth { username: String, password: String },

    #[serde(rename = "register_service")]
    RegisterService { name: String, cmd: Vec<String> },

    #[serde(rename = "list_services")]
    ListServices {},

    #[serde(rename = "start_service")]
    StartService { name: String },

    #[serde(rename = "stop_service")]
    StopService { name: String },

    #[serde(rename = "load_plugin")]
    LoadPlugin { path: String },
}

/// Generic RPC response
#[derive(Debug, Serialize)]
pub struct RpcResponse {
    pub ok: bool,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl RpcResponse {
    pub fn ok() -> Self { Self { ok: true, message: None, data: None } }
    pub fn err(msg: &str) -> Self { Self { ok: false, message: Some(msg.to_string()), data: None } }
}

/// Handle a single JSON request text and return a response object.
pub async fn handle_rpc(
    req_text: &str,
    registry: Arc<Registry>,
    auth: Arc<AuthService>,
    plugin_mgr: Arc<PluginManager>,
) -> RpcResponse {
    let parsed: Result<RpcRequest, _> = serde_json::from_str(req_text);
    match parsed {
        Err(e) => {
            return RpcResponse::err(&format!("invalid request: {}", e));
        }
        Ok(req) => {
            match req {
                RpcRequest::CreateUser { username, password } => {
                    if let Err(e) = auth.create_user(&username, &password).await {
                        return RpcResponse::err(&format!("create_user failed: {}", e));
                    }
                    RpcResponse { ok: true, message: Some("user created".into()), data: None }
                }

                RpcRequest::Auth { username, password } => {
                    match auth.verify_password(&username, &password).await {
                        Ok(true) => RpcResponse { ok: true, message: Some("authenticated".into()), data: None },
                        Ok(false) => RpcResponse::err("invalid username/password"),
                        Err(e) => RpcResponse::err(&format!("auth error: {}", e)),
                    }
                }

                RpcRequest::RegisterService { name, cmd } => {
                    let svc = Service { name: name.clone(), cmd, running: false };
                    if let Err(e) = registry.register(svc).await {
                        return RpcResponse::err(&format!("register_service failed: {}", e));
                    }
                    RpcResponse { ok: true, message: Some(format!("registered {}", name)), data: None }
                }

                RpcRequest::ListServices {} => {
                    let services = registry.list().await;
                    let data = serde_json::to_value(services).ok();
                    RpcResponse { ok: true, message: None, data }
                }

                RpcRequest::StartService { name } => {
                    if let Err(e) = registry.start(&name).await {
                        return RpcResponse::err(&format!("start_service failed: {}", e));
                    }
                    RpcResponse::ok()
                }

                RpcRequest::StopService { name } => {
                    if let Err(e) = registry.stop(&name).await {
                        return RpcResponse::err(&format!("stop_service failed: {}", e));
                    }
                    RpcResponse::ok()
                }

                RpcRequest::LoadPlugin { path } => {
                    if let Err(e) = plugin_mgr.load_plugin(&path).await {
                        return RpcResponse::err(&format!("load_plugin failed: {}", e));
                    }
                    RpcResponse::ok()
                }
            }
        }
    }
}
