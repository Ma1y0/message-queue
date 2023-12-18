use std::env::args;
use tokio::{io::AsyncWriteExt, net::TcpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let sock = TcpSocket::new_v4()?;
    let mut stream = sock.connect(addr).await?;
    let args: Vec<String> = args().collect();

    let message = args[1].as_bytes();

    let _ = stream.write_all(message).await;

    println!("{:?}", args[1].as_bytes());
    Ok(())
}
