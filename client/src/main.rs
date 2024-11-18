use std::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let mut stream = TcpStream::connect(addr).await.unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).await.unwrap();
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).await.unwrap();
        println!("Received from server: {:?}", String::from_utf8_lossy(&buffer[0..size]));
    }

}
