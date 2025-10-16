use crate::config::ServiceConfig;
use anyhow::Result;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub enum ServiceStatus {
    Stopped,
    Running { pid: u32 },
    Failed { reason: String },
}

#[derive(Debug)]
pub struct ServiceEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub child: Arc<RwLock<Option<Child>>>,
}

impl ServiceEntry {
    pub fn new(cfg: &ServiceConfig) -> Self {
        ServiceEntry {
            id: Uuid::new_v4().to_string(),
            name: cfg.name.clone(),
            command: cfg.command.clone(),
            args: cfg.args.clone(),
            child: Arc::new(RwLock::new(None)),
        }
    }
}

#[derive(Debug, Default)]
pub struct ServiceManager {
    services: RwLock<HashMap<String, Arc<ServiceEntry>>>, // name -> entry
    bootstrap_cfg: Vec<ServiceConfig>,
}

impl ServiceManager {
    pub fn new(cfgs: Vec<ServiceConfig>) -> Self {
        ServiceManager {
            services: RwLock::new(HashMap::new()),
            bootstrap_cfg: cfgs,
        }
    }

    pub async fn bootstrap(&self) {
        for cfg in &self.bootstrap_cfg {
            let _ = self.register_service(cfg.clone()).await;
        }
    }

    pub async fn register_service(&self, cfg: ServiceConfig) -> Result<()> {
        let name = cfg.name.clone();
        let entry = Arc::new(ServiceEntry::new(&cfg));
        self.services.write().await.insert(name, entry);
        Ok(())
    }

    pub async fn list_services(&self) -> HashMap<String, ServiceStatus> {
        let mut out = HashMap::new();
        let guard = self.services.read().await;
        for (name, entry) in guard.iter() {
            let c = entry.child.read().await;
            if let Some(child) = c.as_ref() {
                match child.id() {
                    Some(pid) => {
                        out.insert(name.clone(), ServiceStatus::Running { pid });
                    }
                    None => {
                        out.insert(name.clone(), ServiceStatus::Stopped);
                    }
                }
            } else {
                out.insert(name.clone(), ServiceStatus::Stopped);
            }
        }
        out
    }

    pub async fn start_service(&self, name: &str) -> Result<()> {
        let guard = self.services.read().await;
        let entry = guard
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("service {} not found", name))?
            .clone();

        // Spawn process
        let mut cmd = Command::new(&entry.command);
        if !entry.args.is_empty() {
            cmd.args(&entry.args);
        }
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        let mut child = cmd.spawn()?;

        let pid_opt = child.id();

        // store child
        {
            let mut c = entry.child.write().await;
            *c = Some(child);
        }

        if let Some(pid) = pid_opt {
            tracing::info!("Started service {} pid={}", name, pid);
        } else {
            tracing::info!("Started service {} (pid unknown)", name);
        }
        Ok(())
    }

    pub async fn stop_service(&self, name: &str) -> Result<()> {
        let guard = self.services.read().await;
        let entry = guard
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("service {} not found", name))?
            .clone();

        let mut c = entry.child.write().await;
        if let Some(child) = c.as_mut() {
            match child.kill().await {
                Ok(_) => {
                    tracing::info!("Killed service {}", name);
                }
                Err(e) => {
                    tracing::error!("Failed killing {}: {}", name, e);
                }
            }
            *c = None;
            Ok(())
        } else {
            Err(anyhow::anyhow!("service {} not running", name))
        }
    }
}
