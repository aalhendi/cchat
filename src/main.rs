use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;

    let (mut socket, _addr) = listener.accept().await?;

    // After one client connects, start an echo server.
    loop {
        let mut buffer = [0u8; 1024]; // smol 1kB stack buffer
        let bytes_read = socket.read(&mut buffer).await?;
        // Writes every single byte from input buffer to ouput buffer
        socket.write_all(&buffer[..bytes_read]).await?;
    }
}
