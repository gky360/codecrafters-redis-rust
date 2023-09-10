use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer).await {
            Ok(0) => return Ok(()),
            Ok(_) => {
                stream.write_all(b"+PONG\r\n").await?;
                stream.flush().await?;
            }
            Err(e) => {
                println!("Unable to read stream: {}", e);
                return Err(e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
