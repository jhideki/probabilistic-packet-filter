mod bloom;
mod client;
mod filter;
mod server;

use crate::bloom::BloomWrapper;
use crate::client::start_client;
use crate::server::start_server;
use std::sync::Arc;

fn main() {
    // Create bloom filter and populate with blocked IPs
    let mut bloom = BloomWrapper::new(1000, 0.01);
    bloom.insert("192.168.1.100");
    bloom.insert("10.0.0.5");

    let shared_bloom = Arc::new(bloom);

    // Start simulated server with integrated packet filter
    let server_handle = {
        let bloom_clone = shared_bloom.clone();
        std::thread::spawn(move || {
            start_server("127.0.0.1:8080", bloom_clone);
        })
    };

    // Start multiple simulated clients concurrently
    let fake_ips = vec![
        "192.168.1.100", // blocked
        "192.168.1.101",
        "10.0.0.5", // blocked
        "172.16.0.20",
        "203.0.113.50",
    ];

    let mut client_handles = Vec::new();

    for ip in fake_ips {
        let ip_string = ip.to_string();
        let addr = "127.0.0.1:8080".to_string();
        let handle = std::thread::spawn(move || {
            start_client(&addr, &ip_string);
        });
        client_handles.push(handle);
    }

    let _ = server_handle.join();
    for handle in client_handles {
        let _ = handle.join();
    }
}
