use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct TcpStreamManager {
    streams: Vec<(usize, TcpStream)>
}
impl TcpStreamManager {
    fn new() -> Self {
        TcpStreamManager {
            streams: Vec::new(),
        }
    }

    fn add_stream(&mut self, stream: TcpStream) -> usize {
        let id = self.streams.len();
        self.streams.push((id, stream));
        id
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            loop {
                let mut buffer = [0; 1024];
                let size = stream.read(&mut buffer).await.unwrap();
                let message = String::from_utf8_lossy(&buffer[0..size]);
                println!("client message: {}", message);
                stream.write(message.as_bytes()).await.unwrap();
            }
        });
    }
}
