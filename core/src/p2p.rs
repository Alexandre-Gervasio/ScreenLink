use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::error::Error;
use std::io;
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
    connected: bool,
}

struct AppState {
    peers: Arc<Mutex<Vec<Peer>>>,
    local_ip: String,
    selected_peer: usize,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get local IP
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    let app_state = AppState {
        peers: Arc::new(Mutex::new(Vec::new())),
        local_ip: local_ip.clone(),
        selected_peer: 0,
        status: "🚀 KVM Pro v1.0.4 P2P - Iniciando...".to_string(),
    };

    // Start discovery in background
    let peers_clone = app_state.peers.clone();
    let local_ip_clone = local_ip.clone();
    tokio::spawn(async move {
        discovery_loop(peers_clone, local_ip_clone).await;
    });

    // Start TCP server in background
    let peers_clone = app_state.peers.clone();
    tokio::spawn(async move {
        tcp_server(peers_clone).await;
    });

    // Run TUI
    let res = run_app(&mut terminal, app_state).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn get_local_ip() -> Option<String> {
    match local_ip_address::local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(_) => None,
    }
}

async fn discovery_loop(peers: Arc<Mutex<Vec<Peer>>>, local_ip: String) {
    loop {
        // Send broadcast discovery packet
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            let _ = socket.set_broadcast(true);
            let msg = format!("KVM_DISCOVER:{}/{}", local_ip, TCP_PORT);
            let _ = socket.send_to(msg.as_bytes(), BROADCAST_ADDR);
        }

        // Listen for responses
        if let Ok(socket) = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT)) {
            socket.set_read_timeout(Some(std::time::Duration::from_secs(2))).ok();
            let mut buf = [0; 1024];

            if let Ok((size, _addr)) = socket.recv_from(&mut buf) {
                if let Ok(msg) = std::str::from_utf8(&buf[..size]) {
                    if msg.starts_with("KVM_DISCOVER:") {
                        if let Some(peer_info) = msg.strip_prefix("KVM_DISCOVER:") {
                            if let Some((ip, port_str)) = peer_info.rsplit_once('/') {
                                if let Ok(port) = port_str.parse::<u16>() {
                                    if ip != local_ip {
                                        let new_peer = Peer {
                                            name: format!("Peer @ {}", ip),
                                            ip: ip.to_string(),
                                            port,
                                            connected: false,
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

async fn tcp_server(_peers: Arc<Mutex<Vec<Peer>>>) {
    let addr = format!("0.0.0.0:{}", TCP_PORT);
    if let Ok(listener) = TcpListener::bind(&addr).await {
        loop {
            if let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buf = [0u8; 1024];
                    loop {
                        match socket.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                // Echo back para manter conexão viva
                                let _ = socket.write_all(&buf[..n]).await;
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
        }
    }
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> io::Result<()> {
    let mut last_update = std::time::Instant::now();
    
    loop {
        // Render only every 200ms to avoid spam
        if last_update.elapsed().as_millis() > 200 {
            terminal.draw(|f| {
                ui(f, &app_state);
            })?;
            last_update = std::time::Instant::now();
        }

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down => {
                        let peers_len = app_state.peers.blocking_lock().len();
                        if peers_len > 0 {
                            app_state.selected_peer = (app_state.selected_peer + 1) % peers_len;
                        }
                    }
                    KeyCode::Up => {
                        let peers_len = app_state.peers.blocking_lock().len();
                        if peers_len > 0 {
                            app_state.selected_peer =
                                (app_state.selected_peer + peers_len - 1) % peers_len;
                        }
                    }
                    KeyCode::Enter => {
                        let peers_guard = app_state.peers.blocking_lock();
                        if app_state.selected_peer < peers_guard.len() {
                            app_state.status = format!(
                                "✅ Conectado a {} ({}:{})",
                                peers_guard[app_state.selected_peer].name,
                                peers_guard[app_state.selected_peer].ip,
                                peers_guard[app_state.selected_peer].port
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update status
        let peers_guard = app_state.peers.blocking_lock();
        if peers_guard.is_empty() {
            app_state.status = "🔍 Procurando peers na rede...".to_string();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled("🎛️  KVM Pro v1.0.4", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" - P2P Terminal Interface"),
    ])])
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, chunks[0]);

    // Peers list
    let peers_guard = app_state.peers.blocking_lock();
    let peers_text: Vec<Line> = if peers_guard.is_empty() {
        vec![
            Line::from(Span::styled(
                "Nenhum peer descoberto ainda...",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from("Aguardando outros computadores na rede..."),
        ]
    } else {
        peers_guard
            .iter()
            .enumerate()
            .map(|(idx, peer)| {
                let style = if idx == app_state.selected_peer {
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let status = if peer.connected { "✅" } else { "⭕" };
                Line::from(Span::styled(
                    format!("{} {} ({}) - Porta {}", status, peer.name, peer.ip, peer.port),
                    style,
                ))
            })
            .collect()
    };

    let peers_block = Paragraph::new(peers_text)
        .block(
            Block::default()
                .title(" 💻 Peers Disponíveis ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(peers_block, chunks[1]);

    // Footer with controls
    let controls = vec![
        Line::from(Span::styled(
            format!("🌐 IP Local: {}", app_state.local_ip),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            &app_state.status,
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "⬆️ ⬇️ Navegar  |  Enter Conectar  |  Q Sair",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )),
    ];

    let footer = Paragraph::new(controls).block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(footer, chunks[2]);
}
