#![allow(dead_code, unused_variables, unused_imports)]

//! Deliberately worker-limited dummy target backend server.

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_request(mut stream: TcpStream, _addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    if bytes_read == 0 {
        return Ok(());
    }

    // TODO: Implement simple HTTP response handler:
    // 1. Inspect request line from buffer[..bytes_read].
    // 2. If path is "/api/data", respond with:
    //    HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{"status":"ok"}
    // 3. If path is "/login", simulate authentication response.
    // 4. Flush and close stream.
    todo!("Implement backend handle_request()")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(bind_addr).await?;
    println!("[*] Target Backend listening on {}", bind_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_request(stream, addr).await {
                eprintln!("[!] Backend request error from {}: {}", addr, e);
            }
        });
    }
}
