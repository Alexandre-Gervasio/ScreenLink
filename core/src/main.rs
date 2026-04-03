mod network;
mod input;
mod config;
mod screen;
mod security;
mod plugins;
mod clipboard;
mod utils;

use log::info;
use kvm_pro::network::{self, NetworkConfig};
use kvm_pro::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting KVM Pro Server");

    // Load configuration
    let config = Config::default();
    info!("Server config: {:?}", config);

    // Start TCP server
    let network_config = NetworkConfig {
        listen_addr: config.server.host,
        port: config.server.port,
        use_tls: config.security.use_tls,
    };

    network::start_server(network_config).await?;

    Ok(())
}