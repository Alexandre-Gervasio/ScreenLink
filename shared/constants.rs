//! Global constants for KVM Pro

// Version information
pub const APP_NAME: &str = "KVM Pro";
pub const APP_VERSION: &str = "0.1.0";
pub const PROTOCOL_VERSION: u32 = 1;

// Network configuration
pub const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0";
pub const DEFAULT_SERVER_PORT: u16 = 5000;
pub const DEFAULT_DISCOVERY_PORT: u16 = 5001;
pub const DEFAULT_SERVER_HOST: &str = "127.0.0.1";

// Timeouts (in seconds)
pub const CONNECTION_TIMEOUT: u64 = 30;
pub const DISCOVERY_TIMEOUT: u64 = 5;
pub const HEARTBEAT_INTERVAL: u64 = 60;

// Limits
pub const MAX_PAYLOAD_SIZE: usize = 1024 * 1024; // 1 MB
pub const MAX_CONNECTIONS: usize = 1; // Single client per server
pub const BUFFER_SIZE: usize = 4096;

// Input handling
pub const KEY_PRESS_BUFFER_SIZE: usize = 100;
pub const MOUSE_EVENT_BUFFER_SIZE: usize = 100;

// Logging
pub const LOG_LEVEL_DEFAULT: &str = "info";

// Configuration file paths
pub const CONFIG_FILE_NAME: &str = "kvm-pro.toml";
pub const CONFIG_DIR_NAME: &str = ".kvm-pro";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_exist() {
        assert_eq!(DEFAULT_SERVER_PORT, 5000);
        assert_eq!(PROTOCOL_VERSION, 1);
    }
}
