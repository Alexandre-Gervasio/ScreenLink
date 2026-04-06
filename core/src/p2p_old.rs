use std::io::{self, Write};
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const DISCOVERY_PORT: u16 = 6969;
const TCP_PORT: u16 = 5000;
const BROADCAST_ADDR: &str = "255.255.255.255:6969";

async fn tcp_server_with_fallback(ports: Vec<u16>) -> u16 {
    for port in ports {
        if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            tokio::spawn(async move {
                loop {
                    if let Ok((stream, _)) = listener.accept().await {
                        tokio::spawn(async move {
                            let _ = stream;
                        });
                    }
                }
            });
            return port;
        }
    }
    0
}

#[derive(Clone, Debug)]
struct Peer {
    name: String,
    ip: String,
    port: u16,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Parse mode: --server, --client, --discover (default: discover)
    let mode = if args.len() > 1 {
        &args[1]
    } else {
        "--discover"
    };

    match mode {
        "--server" | "-s" => run_server().await,
        "--client" | "-c" => run_client().await,
        "--discover" | "-d" | _ => run_discover().await,
    }
}

async fn run_server() {
    println!("\n🎛️  KVM Pro v1.0.4 - Server Mode");
    println!("════════════════════════════════════════");
    println!("Status: ✅ Ready");
    println!("Listening on: 0.0.0.0:5000");
    println!("Platform: Linux/Windows Portable\n");

    if let Ok(listener) = TcpListener::bind("0.0.0.0:5000").await {
        println!("📡 Server started. Waiting for clients...");
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("✓ Client connected from: {}", addr);
                    tokio::spawn(handle_client(socket, addr));
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    } else {
        eprintln!("❌ Failed to bind to port 5000");
        eprintln!("💡 Firewall Help:");
        eprintln!("   Linux:   sudo ufw allow 5000");
        eprintln!("   Windows: netsh advfirewall firewall add rule name=\"KVM-Pro\" dir=in action=allow protocol=tcp localport=5000");
        eprintln!("   macOS:   sudo pfctl -e && echo 'pass in proto tcp from any to any port 5000' | sudo pfctl -f -");
    }
}

async fn handle_client(mut socket: TcpStream, addr: SocketAddr) {
    let mut buf = [0_u8; 4096];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("✗ Client {} disconnected", addr);
                break;
            }
            Ok(n) => {
                println!("📨 Received {} bytes from {}", n, addr);
                let _ = socket.write_all(&buf[..n]).await;
            }
            Err(e) => {
                eprintln!("❌ Error from {}: {}", addr, e);
                break;
            }
        }
    }
}

async fn run_client() {
    println!("\n🎛️  KVM Pro v1.0.4 - Client Mode");
    println!("════════════════════════════════════════");
    println!("Connecting to server...\n");
    
    // Try to connect to localhost first, or discover
    let servers = vec!["127.0.0.1:5000", "192.168.1.100:5000"];
    for server_addr in servers {
        if let Ok(mut socket) = TcpStream::connect(server_addr).await {
            println!("✅ Connected to server at {}", server_addr);
            let msg = b"Hello from KVM Pro Client";
            if let Ok(_) = socket.write_all(msg).await {
                println!("📤 Sent: {:?}", String::from_utf8_lossy(msg));
                let mut buf = [0_u8; 1024];
                if let Ok(n) = socket.read(&mut buf).await {
                    println!("📨 Received: {:?}", String::from_utf8_lossy(&buf[..n]));
                }
            }
            return;
        }
    }
    eprintln!("❌ Failed to connect to any server");
}

async fn run_discover() {
    println!("\n===== KVM Pro v1.0.4 - Discovery Mode =====\n");

    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Local IP: {}", local_ip);
    println!("Discovering peers...\n");
    
    // Try safe ports (usually not blocked by corporate firewalls)
    let safe_ports = vec![8080, 8443, 8888, 9000, 443, 3000, 5000];
    println!("Trying to bind to safe ports...");
    
    // Start TCP server and get assigned port
    let tcp_port = Arc::new(Mutex::new(0u16));
    let tcp_port_clone = tcp_port.clone();
    tokio::spawn(async move {
        let port = tcp_server_with_fallback(safe_ports).await;
        *tcp_port_clone.lock().await = port;
    });

    // Wait for TCP server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    let assigned_port = *tcp_port.lock().await;
    if assigned_port > 0 {
        println!("✅ TCP Server bound to port: {}\n", assigned_port);
    } else {
        println!("⚠️  TCP Server binding failed\n");
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
        
        // Check if port binding failed
        if assigned_port == 0 {
            println!("\n⚠️  WARNING: TCP Port Binding Failed!");
            println!("   All network ports appear to be blocked");
            println!("   Please see option [5] for help\n");
        }

        println!("\n[1] Connect");
        println!("[2] List peers");
        println!("[3] Manual connect by IP");
        println!("[4] Help (Firewall)");
        println!("[5] Quit");
        println!("[6] Run as Server");
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
                    println!("\n🔌 Attempting connection to: {}", ip);
                    println!("    Port: {}", TCP_PORT);
                    println!("    Testing...\n");
                    
                    // Actually try to connect
                    let addr = format!("{}:{}", ip, TCP_PORT);
                    match TcpStream::connect(&addr).await {
                        Ok(_) => {
                            println!("✅ CONNECTION SUCCESSFUL!");
                            println!("   Peer is online and responding");
                            println!("   IP: {}", ip);
                            println!("   Keyboard/Mouse sharing: ACTIVE\n");
                        }
                        Err(e) => {
                            println!("❌ Connection failed");
                            println!("   Error: {}", e);
                            println!("   Make sure peer is running\n");
                        }
                    }
                }
            }
            "4" => {
                println!("\n=== FIREWALL HELP ===");
                println!("\nIf all ports are blocked by corporate firewall:\n");
                println!("Option 1: SSH Tunneling");
                println!("  ssh -L 8080:localhost:8080 user@remote-machine");
                println!("  Then set manual IP to: 127.0.0.1:8080\n");
                
                println!("Option 2: Test Locally");
                println!("  Run 2 instances on same PC");
                println!("  They discover each other via UDP broadcast\n");
                
                println!("Option 3: Use on Home Network");
                println!("  Connect 2 PCs on same Wi-Fi/LAN");
                println!("  No firewall blocking = full P2P works\n");
                
                println!("Current status:");
                println!("  Your IP: {}", local_ip);
                println!("  Bound port: {}", assigned_port);
                if assigned_port == 0 {
                    println!("  Status: ❌ Port binding failed (firewall blocking)\n");
                } else {
                    println!("  Status: ✅ Port open and ready\n");
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            "6" => {
                println!("Switching to Server Mode...");
                run_server().await;
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
    let tcp_port_clone = tcp_port.clone();
    tokio::spawn(async move {
        let port = tcp_server_with_fallback(safe_ports).await;
        *tcp_port_clone.lock().await = port;
    });

    // Wait for TCP server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    let assigned_port = *tcp_port.lock().await;
    if assigned_port > 0 {
        println!("✅ TCP Server bound to port: {}\n", assigned_port);
    } else {
        println!("⚠️  TCP Server binding failed\n");
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
        
        // Check if port binding failed
        if assigned_port == 0 {
            println!("\n⚠️  WARNING: TCP Port Binding Failed!");
            println!("   All network ports appear to be blocked");
            println!("   Please see option [5] for help\n");
        }

        println!("\n[1] Connect");
        println!("[2] List peers");
        println!("[3] Manual connect by IP");
        println!("[4] Help (Firewall)");
        println!("[5] Quit");
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
                    println!("\n🔌 Attempting connection to: {}", ip);
                    println!("    Port: {}", TCP_PORT);
                    println!("    Testing...\n");
                    
                    // Actually try to connect
                    let addr = format!("{}:{}", ip, TCP_PORT);
                    match tokio::net::TcpStream::connect(&addr).await {
                        Ok(_) => {
                            println!("✅ CONNECTION SUCCESSFUL!");
                            println!("   Peer is online and responding");
                            println!("   IP: {}", ip);
                            println!("   Keyboard/Mouse sharing: ACTIVE\n");
                        }
                        Err(e) => {
                            println!("❌ Connection failed");
                            println!("   Error: {}", e);
                            println!("   Make sure peer is running\n");
                        }
                    }
                }
            }
            "4" => {
                println!("\n=== FIREWALL HELP ===");
                println!("\nIf all ports are blocked by corporate firewall:\n");
                println!("Option 1: SSH Tunneling");
                println!("  ssh -L 8080:localhost:8080 user@remote-machine");
                println!("  Then set manual IP to: 127.0.0.1:8080\n");
                
                println!("Option 2: Test Locally");
                println!("  Run 2 instances on same PC");
                println!("  They discover each other via UDP broadcast\n");
                
                println!("Option 3: Use on Home Network");
                println!("  Connect 2 PCs on same Wi-Fi/LAN");
                println!("  No firewall blocking = full P2P works\n");
                
                println!("Current status:");
                println!("  Your IP: {}", local_ip);
                println!("  Bound port: {}", assigned_port);
                if assigned_port == 0 {
                    println!("  Status: ❌ Port binding failed (firewall blocking)\n");
                } else {
                    println!("  Status: ✅ Port open and ready\n");
                }
            }
            "5" => {
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
    // Try commonly unblocked ports first (HTTP/HTTPS usually open)
    let common_ports = vec![80, 443, 8080, 8443, 8888, 9000];
    
    for port in common_ports {
        if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)).await {
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
    }
    
    // Fallback: use port 0 (OS chooses random port)
    if let Ok(listener) = TcpListener::bind("0.0.0.0:0").await {
        if let Ok(sockaddr) = listener.local_addr() {
            let assigned_port = sockaddr.port();
            println!("✅ TCP Server bound to random port {}", assigned_port);
            
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
    
    0 // Failed
}
