use std::io::{self, Write};
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

const DISCOVERY_PORT: u16 = 6969;
const TCP_PORT: u16 = 5000;
const BROADCAST_ADDR: &str = "255.255.255.255:6969";

#[derive(Clone, Debug)]
struct Peer {
    name: String,
    ip: String,
    port: u16,
}

#[tokio::main]
async fn main() {
    println!("\n===== KVM Pro v1.0.4 - CLI Mode =====\n");

    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Local IP: {}", local_ip);
    println!("Discovering peers...\n");

    // Start TCP server and get assigned port
    let tcp_port = Arc::new(Mutex::new(0u16));
    let tcp_port_clone = tcp_port.clone();
    tokio::spawn(async move {
        let port = tcp_server().await;
        *tcp_port_clone.lock().await = port;
    });

    // Wait for TCP server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let assigned_port = *tcp_port.lock().await;
    if assigned_port > 0 {
        println!("TCP Server listening on port: {}\n", assigned_port);
    }

    let peers = Arc::new(Mutex::new(Vec::new()));

    let peers_clone = peers.clone();
    let local_ip_clone = local_ip.clone();
    let tcp_port_for_discovery = tcp_port.clone();
    tokio::spawn(async move {
        let port = *tcp_port_for_discovery.lock().await;
        discovery_loop(peers_clone, local_ip_clone, port).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    loop {
        println!("\n--- Menu ---");
        let peers_list = peers.lock().await;
        println!("Peers found: {}", peers_list.len());
        for (idx, peer) in peers_list.iter().enumerate() {
            println!("  [{}] {} ({}:{})", idx + 1, peer.name, peer.ip, peer.port);
        }
        drop(peers_list);

        println!("\n[1] Connect");
        println!("[2] List peers");
        println!("[3] Manual connect by IP");
        println!("[4] Quit");
        print!("Choose: ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        match input.trim() {
            "1" => {
                let peers_list = peers.lock().await;
                if peers_list.is_empty() {
                    println!("No peers available");
                } else {
                    print!("Peer number: ");
                    io::stdout().flush().ok();
                    let mut idx_input = String::new();
                    io::stdin().read_line(&mut idx_input).ok();
                    if let Ok(idx) = idx_input.trim().parse::<usize>() {
                        if idx > 0 && idx <= peers_list.len() {
                            let peer = &peers_list[idx - 1];
                            println!("\nConnecting to: {}", peer.name);
                            println!("IP: {}", peer.ip);
                            println!("Port: {}\n", peer.port);
                        }
                    }
                }
            }
            "2" => {
                let peers_list = peers.lock().await;
                println!("\nPeers:\n");
                for peer in peers_list.iter() {
                    println!("  {} ({}:{})", peer.name, peer.ip, peer.port);
                }
            }
            "3" => {
                print!("Enter peer IP address: ");
                io::stdout().flush().ok();
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).ok();
                let ip = ip_input.trim();
                
                if ip.is_empty() {
                    println!("❌ Invalid IP");
                } else {
                    println!("\n✅ Manual connection to: {}", ip);
                    println!("   Using default port: {}", TCP_PORT);
                    println!("   Attempting connection...\n");
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}

fn get_local_ip() -> Option<String> {
    match local_ip_address::local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(_) => None,
    }
}

async fn discovery_loop(peers: Arc<Mutex<Vec<Peer>>>, local_ip: String, tcp_port: u16) {
    loop {
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            let _ = socket.set_broadcast(true);
            let msg = format!("KVM_DISCOVER:{}/{}", local_ip, tcp_port);
            let _ = socket.send_to(msg.as_bytes(), BROADCAST_ADDR);
        }

        if let Ok(socket) = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT)) {
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .ok();
            let mut buf = [0; 1024];

            if let Ok((size, _addr)) = socket.recv_from(&mut buf) {
                if let Ok(msg) = std::str::from_utf8(&buf[..size]) {
                    if msg.starts_with("KVM_DISCOVER:") {
                        if let Some(peer_info) = msg.strip_prefix("KVM_DISCOVER:") {
                            if let Some((ip, port_str)) = peer_info.rsplit_once('/') {
                                if let Ok(port) = port_str.parse::<u16>() {
                                    if ip != local_ip {
                                        let new_peer = Peer {
                                            name: format!("Peer@{}", ip),
                                            ip: ip.to_string(),
                                            port,
                                        };

                                        let mut peers_guard = peers.lock().await;
                                        if !peers_guard.iter().any(|p| p.ip == ip) {
                                            peers_guard.push(new_peer);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

async fn tcp_server() -> u16 {
    // Use port 0 to let OS assign a free port
    let addr = "0.0.0.0:0";
    if let Ok(listener) = TcpListener::bind(&addr).await {
        if let Ok(sockaddr) = listener.local_addr() {
            let assigned_port = sockaddr.port();
            println!("✅ TCP Server bound to port {}", assigned_port);
            
            tokio::spawn(async move {
                loop {
                    if let Ok((mut socket, _)) = listener.accept().await {
                        tokio::spawn(async move {
                            let mut buf = [0u8; 1024];
                            loop {
                                match socket.read(&mut buf).await {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        let _ = socket.write_all(&buf[..n]).await;
                                    }
                                    Err(_) => break,
                                }
                            }
                        });
                    }
                }
            });
            
            return assigned_port;
        }
    }
    
    TCP_PORT // fallback to default
}
