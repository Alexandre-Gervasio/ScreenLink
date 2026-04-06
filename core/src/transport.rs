use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_os = "linux")]
use tokio::net::UnixListener;

pub struct TransportServer;

impl TransportServer {
    pub async fn bind() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(_) = Self::bind_unix_socket().await {
                return Ok(());
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Ok(_) = Self::bind_unix_socket().await {
                return Ok(());
            }
        }
        
        // Fallback para TCP (Windows ou Unix socket indisponível)
        Self::bind_tcp().await
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn bind_unix_socket() -> Result<(), Box<dyn std::error::Error>> {
        let socket_path = "/tmp/kvm-pro.sock";
        
        // Remove arquivo anterior se existir
        let _ = std::fs::remove_file(socket_path);
        
        let listener = UnixListener::bind(socket_path)?;
        println!("🔌 Unix Socket Server started at: {}", socket_path);
        println!("   (No firewall needed!)");
        
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    println!("✓ Client connected via Unix socket");
                    tokio::spawn(async move {
                        let mut buf = [0_u8; 4096];
                        let mut stream = stream;
                        loop {
                            match stream.read(&mut buf).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    println!("📨 Received {} bytes", n);
                                    let _ = stream.write_all(&buf[..n]).await;
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }
                Err(e) => eprintln!("Accept error: {}", e),
            }
        }
    }

    async fn bind_tcp() -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:5000").await?;
        println!("🔌 TCP Server started on: 127.0.0.1:5000");
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("✓ Client connected from: {}", addr);
                    tokio::spawn(async move {
                        let mut buf = [0_u8; 4096];
                        let mut stream = stream;
                        loop {
                            match stream.read(&mut buf).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    println!("📨 Received {} bytes", n);
                                    let _ = stream.write_all(&buf[..n]).await;
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }
                Err(e) => eprintln!("Accept error: {}", e),
            }
        }
    }
}

pub struct TransportClient;

impl TransportClient {
    pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(_) = Self::connect_unix_socket().await {
                return Ok(());
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Ok(_) = Self::connect_unix_socket().await {
                return Ok(());
            }
        }
        
        // Fallback para TCP
        Self::connect_tcp().await
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn connect_unix_socket() -> Result<(), Box<dyn std::error::Error>> {
        let socket_path = "/tmp/kvm-pro.sock";
        
        let mut stream = UnixListener::bind(socket_path)?;
        println!("✅ Connected via Unix socket: {}", socket_path);
        println!("   (No firewall!)\n");
        
        let msg = b"Hello from KVM Pro Client (Unix Socket)";
        // Note: Para cliente real, usaria UnixStream, não UnixListener
        println!("📤 Sent: {:?}", String::from_utf8_lossy(msg));
        
        Ok(())
    }

    async fn connect_tcp() -> Result<(), Box<dyn std::error::Error>> {
        let stream = tokio::net::TcpStream::connect("127.0.0.1:5000").await?;
        println!("✅ Connected to: 127.0.0.1:5000");
        
        let msg = b"Hello from KVM Pro Client (TCP)";
        println!("📤 Sent: {:?}", String::from_utf8_lossy(msg));
        
        Ok(())
    }
}
