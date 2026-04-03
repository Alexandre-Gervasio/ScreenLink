//! KVM Pro - A high-performance KVM software
//!
//! Architecture:
//! - input: Capture keyboard/mouse events
//! - network: TCP/UDP communication
//! - security: TLS encryption
//! - config: Configuration management
//! - plugins: Plugin system

pub mod input;
pub mod network;
pub mod screen;
pub mod security;
pub mod config;
pub mod plugins;
pub mod clipboard;
pub mod utils;

// Re-export protocol types
pub use serde::{Serialize, Deserialize};

/// Main input event type - core of KVM Pro protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseClick { button: u8, down: bool },
    KeyPress { keycode: u32, down: bool },
    Scroll { delta: i32 },
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub allow_clipboard: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 5000,
            use_tls: false,
            allow_clipboard: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server_host: String,
    pub server_port: u16,
    pub use_tls: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".to_string(),
            server_port: 5000,
            use_tls: false,
        }
    }
}
