//! UDP Discovery protocol
//! Automatically discover KVM Pro servers on the network

use tokio::net::UdpSocket;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveryMessage {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub version: String,
}

pub struct DiscoveryServer {
    socket: UdpSocket,
    config: DiscoveryConfig,
}

#[derive(Clone, Debug)]
pub struct DiscoveryConfig {
    pub bind_addr: String,
    pub broadcast_port: u16,
    pub name: String,
    pub version: String,
}

impl DiscoveryServer {
    pub async fn new(config: DiscoveryConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", config.bind_addr, config.broadcast_port);
        let socket = UdpSocket::bind(&addr).await?;
        socket.set_broadcast(true)?;

        Ok(Self { socket, config })
    }

    pub async fn start_responding(
        &self,
        server_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0; 1024];

        loop {
            let (size, peer_addr) = self.socket.recv_from(&mut buf).await?;
            let msg_str = String::from_utf8_lossy(&buf[..size]);

            if msg_str.trim() == "KVM_DISCOVERY_REQUEST" {
                let response = DiscoveryMessage {
                    name: self.config.name.clone(),
                    host: "0.0.0.0".to_string(),
                    port: server_port,
                    version: self.config.version.clone(),
                };

                let response_data = serde_json::to_string(&response)?;
                self.socket.send_to(response_data.as_bytes(), peer_addr).await?;

                log::info!("Discovery response sent to {}", peer_addr);
            }
        }
    }
}

pub struct DiscoveryClient;

impl DiscoveryClient {
    pub async fn discover_servers(
        broadcast_addr: &str,
        port: u16,
    ) -> Result<Vec<DiscoveryMessage>, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.set_broadcast(true)?;

        let target = format!("{}:{}", broadcast_addr, port);
        socket.send_to(b"KVM_DISCOVERY_REQUEST", &target).await?;

        let mut servers = Vec::new();
        let mut buf = [0; 1024];

        // Set a timeout for receiving responses
        let timeout = std::time::Duration::from_secs(2);
        socket.set_read_timeout(Some(timeout))?;

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((size, _)) => {
                    if let Ok(msg) = serde_json::from_slice::<DiscoveryMessage>(&buf[..size]) {
                        servers.push(msg);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break; // Timeout reached
                }
                Err(e) => return Err(Box::new(e)),
            }
        }

        Ok(servers)
    }
}
