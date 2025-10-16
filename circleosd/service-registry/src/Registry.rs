use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{info, warn, error};

use crate::service::{ServiceSpec, RestartPolicy};
use crate::process::{SupervisedProcess, SharedProcess};
use crate::health;

use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Registry {
    // map name -> spec
    services: Arc<RwLock<HashMap<String, ServiceSpec>>>,
    // map name -> supervised process
    processes: Arc<RwLock<HashMap<String, SharedProcess>>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
enum Request {
    #[serde(rename = "register")]
    Register { spec: ServiceSpec },

    #[serde(rename = "unregister")]
    Unregister { name: String },

    #[serde(rename = "start")]
    Start { name: String },

    #[serde(rename = "stop")]
    Stop { name: String },

    #[serde(rename = "list")]
    List {},

    #[serde(rename = "status")]
    Status { name: String },
}

#[derive(Debug, Serialize)]
struct Response {
    ok: bool,
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl Registry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            processes: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn serve(self: Arc<Self>, socket_path: PathBuf) -> Result<()> {
        // ensure old socket removed
        let _ = std::fs::remove_file(&socket_path);
        let listener = UnixListener::bind(&socket_path)?;
        info!("service-registry listening on {}", socket_path.display());

        // restrict socket to owner only (unix)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&socket_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&socket_path, perms)?;
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let reg = self.clone();
            tokio::spawn(async move {
                if let Err(e) = reg.handle_connection(stream).await {
                    error!("connection handler error: {:?}", e);
                }
            });
        }
    }

    async fn handle_connection(self: Arc<Self>, stream: UnixStream) -> Result<()> {
        let (r, mut w) = stream.into_split();
        let mut reader = BufReader::new(r).lines();

        while let Some(line) = reader.next_line().await? {
            if line.trim().is_empty() { continue; }
            tracing::debug!("registry rpc <- {}", line);
            let req: Result<Request, _> = serde_json::from_str(&line);
            let resp = match req {
                Ok(rq) => {
                    match rq {
                        Request::Register { spec } => {
                            let name = spec.name.clone();
                            if self.services.read().contains_key(&name) {
                                Response { ok: false, message: Some("service exists".into()), data: None }
                            } else {
                                self.services.write().insert(name.clone(), spec.clone());
                                Response { ok: true, message: Some("registered".into()), data: None }
                            }
                        }

                        Request::Unregister { name } => {
                            // stop if running
                            let _ = self.stop_service_internal(&name).await;
                            self.services.write().remove(&name);
                            Response { ok: true, message: Some("unregistered".into()), data: None }
                        }

                        Request::Start { name } => {
                            match self.start_service_internal(&name).await {
                                Ok(_) => Response { ok: true, message: Some("started".into()), data: None },
                                Err(e) => Response { ok: false, message: Some(format!("start failed: {}", e)), data: None },
                            }
                        }

                        Request::Stop { name } => {
                            match self.stop_service_internal(&name).await {
                                Ok(_) => Response { ok: true, message: Some("stopped".into()), data: None },
                                Err(e) => Response { ok: false, message: Some(format!("stop failed: {}", e)), data: None },
                            }
                        }

                        Request::List {} => {
                            let svc_list: Vec<_> = self.services.read().values().cloned().collect();
                            Response { ok: true, message: None, data: serde_json::to_value(svc_list).ok() }
                        }

                        Request::Status { name } => {
                            let proc_map = self.processes.read();
                            if let Some(proc) = proc_map.get(&name) {
                                let locked = proc.lock();
                                let running = locked.child.is_some();
                                let since = locked.last_start.map(|i| i.elapsed().as_secs());
                                let data = serde_json::json!({
                                    "name": name,
                                    "running": running,
                                    "restart_count": locked.restart_count,
                                    "last_start_secs_ago": since,
                                });
                                Response { ok: true, message: None, data: Some(data) }
                            } else {
                                Response { ok: false, message: Some("not running".into()), data: None }
                            }
                        }
                    }
                }
                Err(e) => Response { ok: false, message: Some(format!("invalid request: {}", e)), data: None },
            };

            let out = serde_json::to_string(&resp)?;
            w.write_all(out.as_bytes()).await?;
            w.write_all(b"\n").await?;
        }

        Ok(())
    }

    /// Start service by name (public method).
    pub async fn start_service_internal(&self, name: &str) -> Result<()> {
        let spec_opt = { self.services.read().get(name).cloned() };
        let spec = spec_opt.ok_or_else(|| anyhow::anyhow!("service not found"))?;

        // If already has supervised process, and running, skip
        {
            if let Some(proc) = self.processes.read().get(name) {
                let locked = proc.lock();
                if locked.child.is_some() {
                    info!("service {} already running", name);
                    return Ok(());
                }
            }
        }

        // create supervised process
        let mut sp = SupervisedProcess::new(spec.cmd.clone(), spec.restart.clone());
        sp.spawn().await?;
        let shared = Arc::new(parking_lot::Mutex::new(sp));
        self.processes.write().insert(name.to_string(), shared);
        info!("service {} started", name);
        Ok(())
    }

    /// Stop a service by name (public method).
    pub async fn stop_service_internal(&self, name: &str) -> Result<()> {
        if let Some(proc) = self.processes.write().remove(name) {
            let mut locked = proc.lock();
            locked.kill().await?;
            info!("service {} stopped", name);
            Ok(())
        } else {
            // nothing to stop
            Ok(())
        }
    }

    /// Background monitor loop: poll child processes, restart based on policy, simple backoff.
    pub async fn run_monitor(self: Arc<Self>) {
        info!("service-registry monitor running");
        let poll_interval = Duration::from_secs(2);

        loop {
            // collect keys to inspect to avoid holding lock long
            let keys: Vec<String> = {
                let map = self.processes.read();
                map.keys().cloned().collect()
            };

            for name in keys {
                let proc_opt = { self.processes.read().get(&name).cloned() };
                if let Some(shared_proc) = proc_opt {
                    // clone spec to read restart policy if needed
                    let spec_opt = { self.services.read().get(&name).cloned() };
                    if spec_opt.is_none() {
                        // No more spec -> stop
                        let _ = self.stop_service_internal(&name).await;
                        continue;
                    }
                    let spec = spec_opt.unwrap();

                    // Poll
                    let mut locked = shared_proc.lock();
                    match locked.poll_exit().await {
                        Ok(Some(status)) => {
                            info!("service {} exited with {:?}", name, status);
                            // decide restart
                            let should = locked.should_restart(Some(status));
                            if should {
                                // simple backoff: sleep up to current backoff
                                let backoff = locked.backoff;
                                info!("restarting {} after {:?} backoff", name, backoff);
                                tokio::time::sleep(backoff).await;
                                // increase backoff exponentially but cap it
                                locked.backoff = std::cmp::min(backoff * 2, Duration::from_secs(30));
                                if let Err(e) = locked.spawn().await {
                                    error!("failed to restart {}: {:?}", name, e);
                                    // if spawn fails, we may attempt next iteration
                                }
                            } else {
                                // no restart -> remove from processes map
                                drop(locked);
                                let _ = self.processes.write().remove(&name);
                                info!("service {} removed from supervision", name);
                            }
                        }
                        Ok(None) => {
                            // still running; optionally perform health checks
                            drop(locked);
                            // health check placeholder: currently no-op; can extend
                            let healthy = health::check_process_alive(&spec).await;
                            if !healthy {
                                warn!("health check failed for {}", name);
                                // optionally kill or restart
                            }
                        }
                        Err(e) => {
                            error!("error polling {}: {:?}", name, e);
                        }
                    }
                }
            }

            tokio::time::sleep(poll_interval).await;
        }
    }
}
