
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_bind")]
    pub bind_addr: String,
    #[serde(default = "default_plugins_dir")]
    pub plugins_dir: String,
    #[serde(default)]
    pub services: Vec<ServiceConfig>,
    #[serde(default = "default_token")]
    pub auth_token: String,
}

fn default_bind() -> String {
    "0.0.0.0:8080".to_string()
}
fn default_plugins_dir() -> String {
    "/opt/circleos/plugins".to_string()
}
fn default_token() -> String {
    "bolt-default-token".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_addr: default_bind(),
            plugins_dir: default_plugins_dir(),
            services: vec![],
            auth_token: default_token(),
        }
    }
}

impl Config {
    pub fn load_default() -> Option<Config> {
        // Look for /etc/circleos/config.toml or ./config.toml
        let paths = vec!["/etc/circleos/config.toml", "./config.toml"];
        for p in paths {
            let pb = PathBuf::from(p);
            if pb.exists() {
                if let Ok(s) = fs::read_to_string(pb) {
                    match toml::from_str::<Config>(&s) {
                        Ok(cfg) => return Some(cfg),
                        Err(e) => {
                            tracing::error!("Failed to parse config {}: {}", p, e);
                        }
                    }
                }
            }
        }
        None
    }
}
