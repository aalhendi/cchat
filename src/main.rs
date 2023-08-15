use std::error::Error;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;

    loop {
        let (mut socket, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // After one client connects, start an echo server.
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    std::process::exit(0);
                }

                // Writes every single byte from input buffer to ouput buffer
                writer
                    .write_all(line.as_bytes())
                    .await
                    .expect("Could not write line.");
                line.clear();
            }
        });
    }
}
