use std::io::{self, Write};

/// KVM Pro v1.0.1 - Client
/// Connect to a KVM Pro server and forward input events

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎛️  KVM Pro v1.0.1 - Client");
    println!("════════════════════════════════════════");
    println!("Status: ✅ Ready");
    println!("Mode: Client (Input Forwarding)\n");

    print!("Enter server address (default: 127.0.0.1:5000): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let addr = if input.trim().is_empty() {
        "127.0.0.1:5000".to_string()
    } else {
        input.trim().to_string()
    };

    match tokio::net::TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("✓ Connected to server at: {}", addr);
            println!("📡 Ready to forward input events...\n");

            // Simple echo test
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let _ = stream.write_all(b"KVM_PRO_CLIENT_READY").await;

            let mut buf = [0_u8; 1024];
            match stream.read(&mut buf).await {
                Ok(n) => println!("📊 Server response: {} bytes received", n),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Failed to connect to {}: {}", addr, e),
    }

    Ok(())
}
