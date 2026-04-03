//! Configuration management module
//! Handles loading and saving configuration from TOML files

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_clipboard_sync: bool,
    pub enable_screen_sharing: bool,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub server_host: String,
    pub server_port: u16,
    pub auto_connect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub use_tls: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 5000,
                enable_clipboard_sync: true,
                enable_screen_sharing: false,
                max_connections: 1,
            },
            client: ClientConfig {
                server_host: "127.0.0.1".to_string(),
                server_port: 5000,
                auto_connect: false,
            },
            security: SecurityConfig {
                use_tls: false,
                certificate_path: None,
                key_path: None,
            },
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
