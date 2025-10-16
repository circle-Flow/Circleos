use anyhow::Result;
use crate::config::Config;
use axum::http::Request;
use axum::middleware::Next;
use axum::{response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

/// Simple auth middleware that checks for a bearer token in Authorization header.
/// This is MVP — replace with mTLS or a stronger scheme for production.
pub async fn require_token<B>(
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    // Read expected token from extension
    let expected = req
        .extensions()
        .get::<Arc<Config>>()
        .map(|c| c.auth_token.clone())
        .unwrap_or_else(|| "bolt-default-token".to_string());

    if let Some(auth_val) = req.headers().get(axum::http::header::AUTHORIZATION) {
        if let Ok(s) = auth_val.to_str() {
            if s.trim() == format!("Bearer {}", expected) {
                return next.run(req).await;
            }
        }
    }

    let body = Json(json!({"error":"unauthorized"}));
    (axum::http::StatusCode::UNAUTHORIZED, body)
}

/// Placeholder signature verification for plugin packages.
/// For MVP this will just log and accept; replace with real RSA/ECDSA verification.
pub fn verify_plugin_signature(_plugin_path: &str, _sig_path: Option<&str>) -> Result<bool> {
    tracing::warn!("verify_plugin_signature is a stub — replace with real verification!");
    Ok(true)
}
