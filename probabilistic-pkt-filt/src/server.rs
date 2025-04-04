use crate::bloom::BloomWrapper;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

pub fn start_server(addr: &str, bloom: Arc<BloomWrapper>) {
    let listener = TcpListener::bind(addr).expect("Failed to bind server");
    println!("[Server] Listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let fake_ip = String::from_utf8_lossy(&buffer[..size]).trim().to_string();

                        if bloom.contains(&fake_ip) {
                            println!("[Server] 🚫 Blocked client with IP: {}", fake_ip);
                        } else {
                            println!("[Server] ✅ Allowed client with IP: {}", fake_ip);
                            stream.write_all(b"Server ACK").unwrap();
                        }
                    }
                    Err(e) => eprintln!("[Server] Read error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("[Server] Connection failed: {}", e);
            }
        }
    }
}
