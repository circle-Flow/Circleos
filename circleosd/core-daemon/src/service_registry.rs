use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub cmd: Vec<String>,
    pub running: bool,
}

pub struct Registry {
    services: Arc<Mutex<HashMap<String, Service>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Background loop for health checks / restarts (very simple placeholder)
    pub async fn run(self: Arc<Self>) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            // Placeholder: check health, restart crashed services, etc.
            // For now it's a no-op.
        }
    }

    pub async fn register(&self, s: Service) -> Result<()> {
        self.services.lock().await.insert(s.name.clone(), s);
        Ok(())
    }

    pub async fn list(&self) -> Vec<Service> {
        self.services.lock().await.values().cloned().collect()
    }

    /// Start a service by name. Minimal - marks running = true.
    pub async fn start(&self, name: &str) -> Result<()> {
        let mut map = self.services.lock().await;
        if let Some(s) = map.get_mut(name) {
            if s.running {
                return Ok(())
            }
            // TODO: spawn real process via Command; currently simulated
            s.running = true;
        }
        Ok(())
    }

    /// Stop a service by name. Minimal - marks running = false.
    pub async fn stop(&self, name: &str) -> Result<()> {
        let mut map = self.services.lock().await;
        if let Some(s) = map.get_mut(name) {
            s.running = false;
        }
        Ok(())
    }
}
