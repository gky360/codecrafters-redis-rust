use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                loop {
                    let mut msg = String::new();
                    let result = reader.read_line(&mut msg);
                    match result {
                        Ok(_size) => {
                            if msg.as_str() == "ping\r\n" {
                                writer.write_all(b"+PONG\r\n").unwrap();
                                writer.flush().unwrap();
                            }
                        }
                        Err(_err) => break,
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
