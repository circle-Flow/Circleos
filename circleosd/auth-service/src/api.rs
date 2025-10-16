use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{info, error};

use crate::db;
use crate::hash;
use crate::session::SessionStore;
use rusqlite::Connection;

/// RPC Request variants
#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    #[serde(rename = "create_user")]
    CreateUser { username: String, password: String },

    #[serde(rename = "login")]
    Login { username: String, password: String },

    #[serde(rename = "whoami")]
    WhoAmI { token: String },
}

/// RPC Response
#[derive(Debug, Serialize)]
pub struct Response {
    pub ok: bool,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Start serving RPC over the given Unix socket path.
/// Blocks (awaits) until the listener stops.
pub async fn serve(socket_path: std::path::PathBuf, conn: Connection) -> Result<()> {
    // session store
    let sessions = Arc::new(SessionStore::new());
    // spawn a cleanup task for sessions
    {
        let sessions_clone = sessions.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                sessions_clone.cleanup().await;
            }
        });
    }

    let _ = std::fs::remove_file(&socket_path);
    let listener = UnixListener::bind(&socket_path)?;
    // restrict permissions to owner only (600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&socket_path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&socket_path, perms)?;
    }

    info!("auth-service listening on {}", socket_path.display());

    loop {
        let (stream, _) = listener.accept().await?;
        let conn = conn.clone();
        let sessions = sessions.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, conn, sessions).await {
                error!("connection error: {:?}", e);
            }
        });
    }
}

/// Handle a single client connection; reads newline-separated JSON requests and replies linewise.
async fn handle_connection(stream: UnixStream, conn: Connection, sessions: Arc<SessionStore>) -> Result<()> {
    let (r, mut w) = stream.into_split();
    let mut reader = BufReader::new(r).lines();

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        tracing::debug!("auth-service received: {}", line);
        let resp = match serde_json::from_str::<Request>(&line) {
            Ok(req) => process_request(req, &conn, &sessions).await,
            Err(e) => {
                tracing::warn!("invalid request: {}", e);
                Response { ok: false, message: Some(format!("invalid request: {}", e)), data: None }
            }
        };
        let out = serde_json::to_string(&resp)?;
        w.write_all(out.as_bytes()).await?;
        w.write_all(b"\n").await?;
    }

    Ok(())
}

async fn process_request(req: Request, conn: &Connection, sessions: &Arc<SessionStore>) -> Response {
    match req {
        Request::CreateUser { username, password } => {
            // simple checks
            if username.trim().is_empty() || password.is_empty() {
                return Response { ok: false, message: Some("username & password required".into()), data: None };
            }
            // hash password (CPU-bound) — use spawn_blocking
            let pw = password.clone();
            let hash_res = tokio::task::spawn_blocking(move || {
                hash::hash_password(&pw)
            }).await;
            let hash = match hash_res {
                Ok(Ok(h)) => h,
                Ok(Err(e)) => {
                    return Response { ok: false, message: Some(format!("hash error: {}", e)), data: None };
                }
                Err(e) => {
                    return Response { ok: false, message: Some(format!("hash task failed: {}", e)), data: None };
                }
            };

            // insert user in DB (also blocking)
            let username_clone = username.clone();
            let hash_clone = hash.clone();
            let insert_res = tokio::task::spawn_blocking(move || {
                match db::insert_user(conn, &username_clone, &hash_clone) {
                    Ok(id) => Ok(id),
                    Err(e) => Err(format!("{}", e)),
                }
            }).await;

            match insert_res {
                Ok(Ok(_id)) => {
                    Response { ok: true, message: Some("user created".into()), data: None }
                }
                Ok(Err(e)) => {
                    Response { ok: false, message: Some(format!("db error: {}", e)), data: None }
                }
                Err(e) => {
                    Response { ok: false, message: Some(format!("insert task failed: {}", e)), data: None }
                }
            }
        }

        Request::Login { username, password } => {
            // fetch user (blocking)
            let user_res = {
                let username_clone = username.clone();
                tokio::task::spawn_blocking(move || db::get_user_by_username(conn, &username_clone)).await
            };

            let user = match user_res {
                Ok(Ok(Some(u))) => u,
                Ok(Ok(None)) => {
                    return Response { ok: false, message: Some("invalid username/password".into()), data: None };
                }
                Ok(Err(e)) => {
                    return Response { ok: false, message: Some(format!("db error: {}", e)), data: None };
                }
                Err(e) => {
                    return Response { ok: false, message: Some(format!("db task failed: {}", e)), data: None };
                }
            };

            // verify password (blocking)
            let verify_res = {
                let pw = password.clone();
                let hash_str = user.password_hash.clone();
                tokio::task::spawn_blocking(move || hash::verify_password(&pw, &hash_str)).await
            };

            let ok = match verify_res {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => {
                    return Response { ok: false, message: Some(format!("verify error: {}", e)), data: None };
                }
                Err(e) => {
                    return Response { ok: false, message: Some(format!("verify task failed: {}", e)), data: None };
                }
            };

            if ok {
                let token = sessions.create_session(&user.username).await;
                let data = serde_json::json!({ "token": token, "username": user.username });
                Response { ok: true, message: Some("authenticated".into()), data: Some(data) }
            } else {
                Response { ok: false, message: Some("invalid username/password".into()), data: None }
            }
        }

        Request::WhoAmI { token } => {
            let who = sessions.validate(&token).await;
            match who {
                Some(user) => Response { ok: true, message: None, data: Some(serde_json::json!({ "username": user })) },
                None => Response { ok: false, message: Some("invalid or expired token".into()), data: None },
            }
        }
    }
}
