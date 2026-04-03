//! UDP utilities
//! Handles UDP socket operations

use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub async fn create_socket(bind_addr: &str) -> Result<UdpSocket, Box<dyn std::error::Error>> {
    Ok(UdpSocket::bind(bind_addr).await?)
}

pub async fn send_to(
    socket: &UdpSocket,
    data: &[u8],
    target: &SocketAddr,
) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(socket.send_to(data, target).await?)
}

pub async fn recv_from(
    socket: &UdpSocket,
    buf: &mut [u8],
) -> Result<(usize, SocketAddr), Box<dyn std::error::Error>> {
    Ok(socket.recv_from(buf).await?)
}
