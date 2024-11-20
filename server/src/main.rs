use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::TryAcquireError;

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
    let (mut stream, socket_addr) = listener.accept().await.unwrap();

    let (mut reader, mut writer) = tokio::io::split(stream);

    tokio::spawn(async move {
        loop {
            let mut buffer = [0; 1024];
            let i = reader.read(&mut buffer).await.unwrap();
            let message = String::from_utf8_lossy(&buffer[0..i]);
            println!("client message: {}", message);
        }
    });

    tokio::spawn(async move {
        loop {
            writer.write_all("test".as_bytes()).await.unwrap();
        }
    });

    println!("{}", socket_addr);


    loop {

    }
    // loop {
    //     tokio::spawn(async move {
    //         loop {
    //             let mut buffer = [0; 1024];
    //             match stream.read(&mut buffer).await {
    //                 Ok(size) => {
    //                     let message = String::from_utf8_lossy(&buffer[0..size]);
    //                     println!("client message: {}", message);
    //                     stream.write(message.as_bytes()).await.unwrap();
    //                 }
    //                 Err(_) => {}
    //             }
    //         }
    //     });
    // }
}
