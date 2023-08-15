use std::error::Error;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        let tx = tx.clone();
        let mut rx = tx.subscribe(); // Creates a new reciever on every iter
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // After one client connects, start an echo server.
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            std::process::exit(0);
                        }
                        tx.send((line.clone(), addr)).expect("Could not broadcast line.");
                        line.clear();

                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        // Dont echo users message back to them
                        if addr != other_addr {
                            // Writes every single byte from rx buffer to user's screen buffer
                            writer
                                .write_all(msg.as_bytes())
                                .await
                                .expect("Could not write line.");
                            }
                        }
                }
            }
        });
    }
}
