//! TCP server and client implementation

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::time::Instant;
use crate::network::{NetworkConfig, ProtocolHandler};
use crate::InputEvent;
use log::{info, warn, error, debug};

const CONNECT_TIMEOUT_SECS: u64 = 5;
const READ_TIMEOUT_SECS: u64 = 30;
const KEEPALIVE_INTERVAL_SECS: u64 = 10;

/// TCP Server configuration
pub struct TcpServerConfig {
    pub listen_addr: String,
    pub port: u16,
    pub max_retries: u32,
}

impl Default for TcpServerConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".to_string(),
            port: 5000,
            max_retries: 3,
        }
    }
}

pub async fn start_server(config: NetworkConfig) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.listen_addr, config.port);
    
    let listener = TcpListener::bind(&addr).await?;
    info!("KVM Pro server listening on {} (Latency target: < 5ms)", addr);

    let mut client_count = 0;

    loop {
        match listener.accept().await {
            Ok((socket, peer_addr)) => {
                client_count += 1;
                info!("Client #{} connected: {}", client_count, peer_addr);
                
                let client_id = client_count;
                tokio::spawn(async move {
                    match handle_client(socket, client_id).await {
                        Ok(_) => {
                            info!("Client #{} disconnected gracefully", client_id);
                        }
                        Err(e) => {
                            error!("Client #{} error: {}", client_id, e);
                        }
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle_client(
    mut socket: TcpStream,
    client_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Set TCP settings for low latency
    socket.set_nodelay(true)?; // Disable Nagle's algorithm
    
    let (mut reader, mut writer) = socket.split();
    let mut buf = [0; 1024];
    let mut event_count = 0;
    let mut latency_sum: u128 = 0;

    loop {
        // Set read timeout
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(READ_TIMEOUT_SECS),
            reader.read(&mut buf),
        )
        .await
        {
            Ok(Ok(0)) => {
                info!(
                    "Client #{} disconnected after {} events (avg latency: {:?}ms)",
                    client_id,
                    event_count,
                    if event_count > 0 { latency_sum / event_count as u128 } else { 0 }
                );
                break;
            }
            Ok(Ok(size)) => {
                let start = Instant::now();
                
                match ProtocolHandler::deserialize_event(&buf[..size]) {
                    Ok(event) => {
                        let deserialize_time = start.elapsed();
                        
                        // Measure injection time
                        let inject_start = Instant::now();
                        
                        debug!("Client #{} - Event: {:?}", client_id, event);
                        
                        // Inject the event
                        #[cfg(target_os = "linux")]
                        {
                            if let Err(e) = crate::input::inject::inject_event(&event).await {
                                error!("Client #{} - Failed to inject event: {}", client_id, e);
                            }
                        }
                        
                        let total_latency = start.elapsed();
                        latency_sum += total_latency.as_millis();
                        event_count += 1;
                        
                        if event_count % 1000 == 0 {
                            let avg_latency =
                                latency_sum / event_count as u128;
                            if avg_latency > 10 {
                                warn!(
                                    "Client #{} - High latency detected: {}ms (deserialize: {:?}ms)",
                                    client_id,
                                    avg_latency,
                                    deserialize_time.as_millis()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Client #{} - Failed to deserialize event: {}",
                            client_id, e
                        );
                    }
                }
            }
            Ok(Err(e)) => {
                error!("Client #{} - Read error: {}", client_id, e);
                break;
            }
            Err(_) => {
                warn!("Client #{} - Read timeout (no data for {}s)", client_id, READ_TIMEOUT_SECS);
                break;
            }
        }
    }

    Ok(())
}

/// TCP Client with automatic reconnection
pub struct TcpClient {
    host: String,
    port: u16,
    sender: tokio::sync::mpsc::Sender<InputEvent>,
    latency_ms: Arc<tokio::sync::Mutex<u128>>,
}

impl TcpClient {
    pub async fn connect(
        host: &str,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let latency_ms = Arc::new(tokio::sync::Mutex::new(0));
        let latency_clone = latency_ms.clone();
        
        let host_clone = host.to_string();

        tokio::spawn(async move {
            if let Err(e) = handle_client_connection(&host_clone, port, receiver, latency_clone).await {
                error!("Client connection error: {}", e);
            }
        });

        info!("KVM Pro client connecting to {}:{}", host, port);

        Ok(Self {
            host: host.to_string(),
            port,
            sender,
            latency_ms,
        })
    }

    pub async fn send_event(&self, event: InputEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.sender.send(event).await?;
        Ok(())
    }

    pub async fn get_latency(&self) -> u128 {
        *self.latency_ms.lock().await
    }
}

async fn handle_client_connection(
    host: &str,
    port: u16,
    mut receiver: tokio::sync::mpsc::Receiver<InputEvent>,
    latency_ms: Arc<tokio::sync::Mutex<u128>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5;

    loop {
        let addr = format!("{}:{}", host, port);

        // Connect with timeout and retry logic
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(CONNECT_TIMEOUT_SECS),
            TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(mut socket)) => {
                // Set TCP options for low latency
                socket.set_nodelay(true)?;
                retry_count = 0;
                info!("Connected to {}", addr);

                let (_, mut writer) = socket.split();
                let mut event_count = 0;

                while let Some(event) = receiver.recv().await {
                    let start = Instant::now();

                    match ProtocolHandler::serialize_event(&event) {
                        Ok(data) => {
                            match writer.write_all(&data).await {
                                Ok(_) => {
                                    event_count += 1;
                                    let latency = start.elapsed().as_millis();
                                    *latency_ms.lock().await = latency;

                                    if latency > 10 {
                                        warn!(
                                            "High latency sending event: {}ms",
                                            latency
                                        );
                                    }

                                    if event_count % 1000 == 0 {
                                        info!(
                                            "Sent {} events (current latency: {}ms)",
                                            event_count, latency
                                        );
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to send event: {}", e);
                                    break; // Reconnect
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to serialize event: {}", e);
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                retry_count += 1;
                if retry_count > MAX_RETRIES {
                    return Err(format!("Failed to connect after {} retries: {}", MAX_RETRIES, e).into());
                }
                warn!(
                    "Connection failed (retry {}/{}): {}",
                    retry_count, MAX_RETRIES, e
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Err(_) => {
                retry_count += 1;
                if retry_count > MAX_RETRIES {
                    return Err("Connection timeout after max retries".into());
                }
                warn!(
                    "Connection timeout (retry {}/{}): retrying in 1s",
                    retry_count, MAX_RETRIES
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_protocol_handler() {
        let event = InputEvent::KeyPress {
            keycode: 65,
            down: true,
        };

        let encoded = ProtocolHandler::serialize_event(&event).unwrap();
        let decoded = ProtocolHandler::deserialize_event(&encoded).unwrap();

        assert!(matches!(
            decoded,
            InputEvent::KeyPress {
                keycode: 65,
                down: true
            }
        ));
    }

    #[tokio::test]
    async fn test_latency_measurement() {
        let event = InputEvent::MouseMove { x: 100, y: 200 };
        
        let start = Instant::now();
        let _ = ProtocolHandler::serialize_event(&event);
        let latency = start.elapsed();
        
        // Serialization should be fast (<5ms)
        assert!(latency.as_millis() < 5);
    }
}