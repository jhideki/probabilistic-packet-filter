use std::io::Write;
use std::net::TcpStream;

pub fn start_client(addr: &str, fake_ip: &str) {
    std::thread::sleep(std::time::Duration::from_millis(100));

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("[Client] Connected to {}", addr);
            stream
                .write_all(fake_ip.as_bytes())
                .expect("Failed to write");
        }
        Err(e) => {
            eprintln!("[Client] Connection failed: {}", e);
        }
    }
}
