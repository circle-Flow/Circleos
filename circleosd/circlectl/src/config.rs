use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub plugin_socket: String,
    pub registry_socket: String,
    pub auth_socket: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            plugin_socket: "/tmp/plugin-manager.sock".to_string(),
            registry_socket: "/tmp/service-registry.sock".to_string(),
            auth_socket: "/tmp/auth-service.sock".to_string(),
        }
    }
}

impl CliConfig {
    pub fn load_or_default() -> Self {
        // attempt to read ~/.circleosd/config.toml (not implemented — default used)
        Self::default()
    }
}
