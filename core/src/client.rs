mod network;
mod input;
mod config;
mod screen;
mod security;
mod plugins;
mod clipboard;
mod utils;

use log::info;
use kvm_pro::network::{self, discovery};
use kvm_pro::config::Config;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting KVM Pro Client");

    // Load configuration
    let config = Config::default();
    info!("Client config: {:?}", config);

    // Try to auto-discover server
    let host = if config.client.auto_connect {
        match auto_discover_server().await {
            Some(host) => {
                info!("Auto-discovered server at {}", host);
                host
            }
            None => {
                info!("Failed to auto-discover server, using configured host");
                config.client.server_host
            }
        }
    } else {
        config.client.server_host
    };

    // Connect to server
    let tx = network::connect_client(&host, config.client.server_port, config.security.use_tls).await?;
    info!("Connected to server at {}:{}", host, config.client.server_port);

    // Start capturing events and sending them
    tokio::select! {
        _ = input::capture::capture_events() => {
            info!("Event capture ended");
        }
    }

    Ok(())
}

async fn auto_discover_server() -> Option<String> {
    match discovery::DiscoveryClient::discover_servers("255.255.255.255", 5000).await {
        Ok(servers) => {
            if let Some(server) = servers.first() {
                return Some(server.host.clone());
            }
        }
        Err(e) => {
            eprintln!("Discovery failed: {}", e);
        }
    }
    None
}
