//! Network module - TCP/UDP communication
//! Handles server/client connections and message delivery

pub mod tcp;
pub mod udp;
pub mod discovery;
pub mod protocol_handler;

use std::net::SocketAddr;
use tokio::sync::mpsc;

pub use self::protocol_handler::ProtocolHandler;
pub use self::discovery::{DiscoveryServer, DiscoveryClient, DiscoveryConfig, DiscoveryMessage};

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub port: u16,
    pub use_tls: bool,
}

pub type MessageSender = mpsc::Sender<crate::InputEvent>;
pub type MessageReceiver = mpsc::Receiver<crate::InputEvent>;

pub async fn start_server(config: NetworkConfig) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting KVM Pro server on {}:{}", config.listen_addr, config.port);
    
    tcp::start_server(config).await
}

pub async fn connect_client(
    host: &str,
    port: u16,
    use_tls: bool,
) -> Result<MessageSender, Box<dyn std::error::Error>> {
    log::info!("Connecting to server at {}:{}", host, port);
    
    tcp::connect_client(host, port, use_tls).await
}
