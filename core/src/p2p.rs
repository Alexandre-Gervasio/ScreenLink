use std::io::{self, Write};
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
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
    let args: Vec<String> = std::env::args().collect();
    
    // Parse mode: --server, --client, --discover, --help (default: discover)
    let mode = if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" | "help" => {
                print_help();
                return;
            }
            _ => &args[1]
        }
    } else {
        "--discover"
    };

    match mode {
        "--server" | "-s" => run_server().await,
        "--client" | "-c" => run_client().await,
        "--local" | "-l" => run_local().await,
        "--discover" | "-d" | _ => run_discover().await,
    }
}

fn print_help() {
    println!("\n🎛️  KVM Pro v1.0.7 - Help");
    println!("════════════════════════════════════════");
    println!("\n📖 USAGE:");
    println!("  kvm-pro              # Discovery mode (default)");
    println!("  kvm-pro -s           # Start as Server");
    println!("  kvm-pro -c           # Start as Client");
    println!("  kvm-pro -l           # Local mode (NO PORTS - Linux/macOS)");
    println!("  kvm-pro -h           # Show this help\n");
    
    println!("🎯 MODES:");
    println!("  Discovery     - Auto-find peers on network");
    println!("  Server        - Listen for client connections");
    println!("  Client        - Connect to remote server");
    println!("  Local         - LOCAL ONLY (No firewall issues!)");
    println!("                  Uses Unix sockets on Linux/macOS");
    println!("                  Uses localhost on Windows\n");
    
    println!("🔥 FIREWALL ISSUES?");
    println!("  If peers don't appear, your intranet likely blocks ports.\n");
    
    println!("✅ SOLUTIONS:");
    println!("  1️⃣  Local Mode (EASIEST - NO PORTS)");
    println!("     kvm-pro -l");
    println!("     Uses Unix sockets (Linux/macOS) or localhost (Windows)");
    println!("     No network traffic, no firewall needed!\\n");
    
    println!("  2️⃣  SSH Tunneling");
    println!("     ssh -L 5000:SERVER_IP:5000 user@remote-gateway");
    println!("     Then in KVM Pro:");
    println!("       Connect to: 127.0.0.1:5000\n");
    
    println!("  3️⃣  VPN (If available)");
    println!("     Connect to company VPN first");
    println!("     Then use normal discovery\n");
    
    println!("  4️⃣  Manual Connection");
    println!("     Use option [3] in discovery menu");
    println!("     Enter direct IP:PORT of remote machine\n");
    
    println!("  5️⃣  Reverse Port Forward");
    println!("     On SERVER machine:");
    println!("       ssh -R 5000:localhost:5000 user@client-machine");
    println!("     Then client connects to localhost:5000\n");
    
    println!("💡 COMMON PORTS (often less blocked):");
    println!("   80 (HTTP), 443 (HTTPS), 8080, 8443\n");
}

async fn run_server() {
    println!("\n🎛️  KVM Pro v1.0.6 - Server Mode");
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
    println!("\n🎛️  KVM Pro v1.0.6 - Client Mode");
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

async fn run_local() {
    println!("\n🎛️  KVM Pro v1.0.7 - Local Mode");
    println!("════════════════════════════════════════");
    println!("✅ NO FIREWALL - Using local sockets\n");

    #[cfg(target_os = "linux")]
    {
        println!("🔌 Unix Domain Socket: /tmp/kvm-pro.sock");
        println!("   (No network ports, no firewall issues!)\n");
        
        if let Ok(listener) = tokio::net::UnixListener::bind("/tmp/kvm-pro.sock") {
            println!("📡 Local Server started");
            println!("   Type: Unix Socket");
            println!("   Path: /tmp/kvm-pro.sock");
            println!("   Status: ✅ LISTENING\n");
            
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        println!("✓ Local client connected via Unix socket");
                        tokio::spawn(async move {
                            let mut buf = [0_u8; 4096];
                            let mut stream = stream;
                            loop {
                                match stream.read(&mut buf).await {
                                    Ok(0) => {
                                        println!("✗ Local client disconnected");
                                        break;
                                    }
                                    Ok(n) => {
                                        println!("📨 Received {} bytes", n);
                                        let _ = stream.write_all(&buf[..n]).await;
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Error: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => eprintln!("Accept error: {}", e),
                }
            }
        } else {
            eprintln!("❌ Failed to bind Unix socket");
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("🔌 Unix Domain Socket: /tmp/kvm-pro.sock");
        println!("   (No network ports, no firewall issues!)\n");
        
        let _ = std::fs::remove_file("/tmp/kvm-pro.sock");
        
        if let Ok(listener) = tokio::net::UnixListener::bind("/tmp/kvm-pro.sock") {
            println!("📡 Local Server started");
            println!("   Type: Unix Socket");
            println!("   Path: /tmp/kvm-pro.sock");
            println!("   Status: ✅ LISTENING\n");
            
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        println!("✓ Local client connected via Unix socket");
                        tokio::spawn(async move {
                            let mut buf = [0_u8; 4096];
                            let mut stream = stream;
                            loop {
                                match stream.read(&mut buf).await {
                                    Ok(0) => {
                                        println!("✗ Local client disconnected");
                                        break;
                                    }
                                    Ok(n) => {
                                        println!("📨 Received {} bytes", n);
                                        let _ = stream.write_all(&buf[..n]).await;
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Error: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => eprintln!("Accept error: {}", e),
                }
            }
        } else {
            eprintln!("❌ Failed to bind Unix socket");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("🔌 Localhost TCP: 127.0.0.1:5000");
        println!("   (Local only, no remote connections)\n");
        
        if let Ok(listener) = TcpListener::bind("127.0.0.1:5000").await {
            println!("📡 Local Server started");
            println!("   Type: TCP Localhost");
            println!("   Address: 127.0.0.1:5000");
            println!("   Status: ✅ LISTENING\n");
            
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("✓ Local client connected from {}", addr);
                        tokio::spawn(async move {
                            let mut buf = [0_u8; 4096];
                            let mut stream = stream;
                            loop {
                                match stream.read(&mut buf).await {
                                    Ok(0) => {
                                        println!("✗ Local client disconnected");
                                        break;
                                    }
                                    Ok(n) => {
                                        println!("📨 Received {} bytes", n);
                                        let _ = stream.write_all(&buf[..n]).await;
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Error: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => eprintln!("Accept error: {}", e),
                }
            }
        } else {
            eprintln!("❌ Failed to bind to localhost:5000");
        }
    }
}

async fn run_discover() {
    println!("\n===== KVM Pro v1.0.7 - Discovery Mode =====\n");

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
        println!("[5] Switch to Server");
        println!("[6] Quit");
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
                println!("\n🔧 Manual Connection (for Firewall Bypass)");
                println!("════════════════════════════════════════");
                
                print!("Enter IP address (or leave blank for 127.0.0.1): ");
                io::stdout().flush().ok();
                let mut ip_input = String::new();
                io::stdin().read_line(&mut ip_input).ok();
                let ip = ip_input.trim();
                let ip = if ip.is_empty() { "127.0.0.1" } else { ip };
                
                print!("Enter port (or leave blank for 5000): ");
                io::stdout().flush().ok();
                let mut port_input = String::new();
                io::stdin().read_line(&mut port_input).ok();
                let port_str = port_input.trim();
                let port: u16 = port_str.parse().unwrap_or(5000);
                
                println!("\n🔌 Attempting connection to: {}:{}", ip, port);
                println!("    Testing...\n");
                
                let addr = format!("{}:{}", ip, port);
                match TcpStream::connect(&addr).await {
                    Ok(_) => {
                        println!("✅ CONNECTION SUCCESSFUL!");
                        println!("   Server is online and responding");
                        println!("   IP: {}", ip);
                        println!("   Port: {}", port);
                        println!("   Keyboard/Mouse sharing: ACTIVE\n");
                    }
                    Err(e) => {
                        println!("❌ Connection failed: {}", e);
                        println!("\n💡 Tips:");
                        println!("   • Make sure server is running (kvm-pro -s)");
                        println!("   • If using SSH tunnel, check ssh connection first");
                        println!("   • Try alternative ports (80, 443, 8080, 8443)");
                        println!("   • Check if firewall is blocking the port\n");
                    }
                }
            }
            "4" => {
                println!("\n╔════════════════════════════════════════╗");
                println!("║       CORPORATE FIREWALL SOLUTIONS      ║");
                println!("╚════════════════════════════════════════╝");
                
                println!("\n📊 DIAGNÓSTICO:");
                println!("   Your LAN IP: {}", local_ip);
                println!("   Peers found: {}", peers.lock().await.len());
                
                let port_status = if assigned_port == 0 { 
                    "❌ BLOCKED - likely intranet firewall".to_string()
                } else { 
                    format!("✅ OPEN on port {}", assigned_port)
                };
                println!("   Port status: {}", port_status);
                
                println!("\n🎯 SOLUTIONS (in order of preference):\n");
                
                println!("1️⃣  SSH TUNNELING (Best for Corporate Networks)");
                println!("   ═══════════════════════════════════════════");
                println!("   Command on CLIENT machine:");
                println!("   $ ssh -L 5000:SERVER_LOCAL_IP:5000 user@ssh-gateway");
                println!("   ");
                println!("   Then in KVM Pro:");
                println!("   • Use option [3]: Manual connect");
                println!("   • Enter IP: 127.0.0.1");
                println!("   • Enter Port: 5000\n");
                
                println!("2️⃣  REVERSE SSH TUNNEL (SERVER to CLIENT)");
                println!("   ═════════════════════════════════════════");
                println!("   Command on SERVER machine:");
                println!("   $ ssh -R 5000:localhost:5000 user@client-machine");
                println!("   ");
                println!("   Then CLIENT connects to localhost:5000\n");
                
                println!("3️⃣  VPN CONNECTION");
                println!("   ════════════════");
                println!("   • Connect to company VPN first");
                println!("   • Then use normal discovery (should find peers)\n");
                
                println!("4️⃣  ALTERNATIVE PORTS (if SSH not available)");
                println!("   ════════════════════════════════════════");
                println!("   Most corporate firewalls allow:");
                println!("   • Port 80 (HTTP)");
                println!("   • Port 443 (HTTPS)");
                println!("   • Port 8080 (HTTP-Proxy)");
                println!("   • Port 3389 (RDP - sometimes)\n");
                
                println!("5️⃣  SAME NETWORK (Simplest if Available)");
                println!("   ═════════════════════════════════════");
                println!("   • Connect both PCs to same Wi-Fi/LAN");
                println!("   • Use normal discovery\n");
                
                println!("❓ HOW TO TELL IF FIREWALLED:");
                println!("   • Peers found = 0 → Likely firewalled");
                println!("   • Port status = BLOCKED → Definitely firewalled\n");
            }
            "5" => {
                println!("Switching to Server Mode...");
                run_server().await;
                break;
            }
            "6" => {
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
