use std::sync::Arc;

use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::RwLock,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on port :8080");

    // File to store the events
    let file = Arc::new(RwLock::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("event.txt")
            .await?,
    ));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let file_clone = Arc::clone(&file);

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(_) => break,
                };

                let mut file_a = file_clone.write().await;

                file_a.write_all(&buf[..n]).await;
            }
        });
    }
}
