use crate::config::Config;
use crate::plugin::PluginManager;
use crate::service::{ServiceManager, ServiceStatus};
use anyhow::Result;
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::security::require_token;

#[derive(Serialize)]
struct Health {
    status: String,
    version: String,
}

pub fn router(
    svc_mgr: Arc<ServiceManager>,
    plugin_mgr: Arc<PluginManager>,
    config: Config,
) -> Router {
    let config_arc = Arc::new(config);
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    Router::new()
        .route("/health", get(health))
        .route("/services", get(list_services))
        .route("/service/start", post(start_service))
        .route("/service/stop", post(stop_service))
        .route("/plugins", get(list_plugins))
        .route("/plugin/load", post(load_plugin))
        .route("/plugin/call", post(call_plugin))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(Extension(svc_mgr.clone()))
                .layer(Extension(plugin_mgr.clone()))
                .layer(Extension(config_arc.clone()))
        )
        // simple token auth middleware applied to all routes
        .route_layer(axum::middleware::from_fn_with_state(config_arc.clone(), require_token))
}

async fn health(
    Extension(cfg): Extension<Arc<Config>>,
) -> impl IntoResponse {
    let h = Health {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    };
    (StatusCode::OK, axum::Json(h))
}

async fn list_services(
    Extension(svc_mgr): Extension<Arc<ServiceManager>>,
) -> impl IntoResponse {
    let map = svc_mgr.list_services().await;
    // convert enum to simple serializable form
    let simple: std::collections::HashMap<String, serde_json::Value> = map
        .into_iter()
        .map(|(n, s)| {
            let v = match s {
                ServiceStatus::Stopped => serde_json::json!({"status":"stopped"}),
                ServiceStatus::Running { pid } => serde_json::json!({"status":"running","pid":pid}),
                ServiceStatus::Failed { reason } => serde_json::json!({"status":"failed","reason":reason}),
            };
            (n, v)
        })
        .collect();
    (StatusCode::OK, Json(simple))
}

#[derive(Deserialize)]
struct ServiceAction {
    name: String,
}

async fn start_service(
    Extension(svc_mgr): Extension<Arc<ServiceManager>>,
    Json(payload): Json<ServiceAction>,
) -> impl IntoResponse {
    match svc_mgr.start_service(&payload.name).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"result":"ok"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error":format!("{}", e)}))),
    }
}

async fn stop_service(
    Extension(svc_mgr): Extension<Arc<ServiceManager>>,
    Json(payload): Json<ServiceAction>,
) -> impl IntoResponse {
    match svc_mgr.stop_service(&payload.name).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"result":"ok"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error":format!("{}", e)}))),
    }
}

async fn list_plugins(
    Extension(plugin_mgr): Extension<Arc<PluginManager>>,
) -> impl IntoResponse {
    let plugins = plugin_mgr.list_plugins().await;
    (StatusCode::OK, Json(plugins))
}

#[derive(Deserialize)]
struct LoadPluginReq {
    path: String,
}

async fn load_plugin(
    Extension(plugin_mgr): Extension<Arc<PluginManager>>,
    Json(payload): Json<LoadPluginReq>,
) -> impl IntoResponse {
    match plugin_mgr.load_plugin(&payload.path).await {
        Ok(p) => (StatusCode::OK, Json(p)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":format!("{}", e)}))),
    }
}

#[derive(Deserialize)]
struct CallPluginReq {
    id: String,
    input: String,
}

async fn call_plugin(
    Extension(plugin_mgr): Extension<Arc<PluginManager>>,
    Json(payload): Json<CallPluginReq>,
) -> impl IntoResponse {
    match plugin_mgr.call_plugin(&payload.id, &payload.input).await {
        Ok(resp) => (StatusCode::OK, Json(resp)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error":format!("{}", e)}))),
    }
}
