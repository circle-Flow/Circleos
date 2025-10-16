use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Simple restart policy for supervised services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

/// Definition of a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: String,
    pub cmd: Vec<String>, // first element is executable
    #[serde(default)]
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub restart: RestartPolicy,
    #[serde(default)]
    pub max_restarts: Option<u32>, // optional limit in a time window
    #[serde(default)]
    pub health_check: Option<String>, // placeholder e.g. "http://127.0.0.1:8080/health"
}

impl ServiceSpec {
    pub fn id(&self) -> String {
        // stable id derived from name
        format!("{}-{}", self.name, Uuid::new_v4().to_string())
    }
}
