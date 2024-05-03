use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

struct Client {
    name: String,
    addr: SocketAddr,
    sender: mpsc::Sender<String>,
}

impl Client {
    async fn new(name: String, addr: SocketAddr, sender: mpsc::Sender<String>, mut stream: TcpStream) {
        let (mut tx, rx) = mpsc::channel::<String>(32);

        let client = Client {
            name: name.clone(),
            addr,
            sender: tx.clone(),
        };

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("{} has disconnected", name);
                        break;
                    }
                    Ok(n) => n,
                    Err(_) => {
                        println!("{} has encountered an error and disconnected", name);
                        break;
                    }
                };

                let msg = String::from_utf8_lossy(&buf[0..n]).to_string();
                tx.send(msg.clone()).await.unwrap();
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(_) = stream.write_all(msg.as_bytes()).await {
                    break;
                }
            }
        });
    }
}
fn main() {

}
