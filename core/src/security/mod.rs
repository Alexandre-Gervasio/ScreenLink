//! Security module - TLS and encryption
//! Handles SSL/TLS connections and certificate management

pub mod tls;

use std::path::Path;
use rustls::{ServerConfig, ClientConfig};

pub use tls::{create_server_config, create_client_config, load_certificates, load_private_key};

pub struct SecurityManager {
    use_tls: bool,
}

impl SecurityManager {
    pub fn new(use_tls: bool) -> Self {
        Self { use_tls }
    }

    pub fn is_tls_enabled(&self) -> bool {
        self.use_tls
    }

    pub fn generate_self_signed_cert(
        _cert_path: &Path,
        _key_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::warn!("Self-signed certificate generation not yet implemented");
        log::warn!("Use external tools like OpenSSL to generate certificates");
        Ok(())
    }
}
