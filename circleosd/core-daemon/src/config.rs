use std::path::PathBuf;

/// Simple in-memory config. Extend to read from /etc/circleosd/circleosd.conf
#[derive(Clone, Debug)]
pub struct Config {
    pub socket_path: String,
    pub log_level: String,
}

impl Config {
    pub fn load_default() -> Self {
        // In a real system, parse a toml file from /etc/circleosd/circleosd.conf
        Self {
            socket_path: "/tmp/circleosd.sock".to_string(),
            log_level: "info".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn from_path(_p: PathBuf) -> Self {
        // TODO: implement reading the file
        Self::load_default()
    }
}
