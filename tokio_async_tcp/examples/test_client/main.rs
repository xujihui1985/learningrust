use tokio::net::{TcpStream};
use tokio::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> MyResult<()> {
    let addr = "::1:1026";
    let mut socket = TcpStream::connect(addr).await?;

    println!("chunk1");
    socket.write_all(b"37950").await?;

    delay_for(Duration::from_millis(500)).await;
    println!("chunk2");
    socket.write_all(b"22").await?;

    delay_for(Duration::from_millis(500)).await;
    println!("chunk3");
    socket.write_all(b"\r\n").await?;

    Ok(())
}
