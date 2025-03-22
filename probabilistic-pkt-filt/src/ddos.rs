use crate::packet_filter::{PacketFilter, PacketFilterHashSet};
use crate::utils::random_ip;
use std::time::Instant;

pub struct Ddos;

impl Ddos {
    /// Measures memory usage of inserting IPs into a HashSet-based filter
    pub fn test_hashset_mem(blacklist_size: usize) {
        println!("========================================");
        println!("HashSet Insertion Memory Test");
        println!("Inserting {} IPs...", blacklist_size);

        let start = Instant::now();

        // Insert generated IPs directly into the HashSet
        let mut filter = PacketFilterHashSet::new();
        for _ in 0..blacklist_size {
            let ip = random_ip();
            filter.add_blacklisted_ip(ip);
        }

        let elapsed = start.elapsed();
        println!("Insertion completed in {:.3}s", elapsed.as_secs_f64());
        println!("========================================");
    }

    /// Measures memory usage of inserting IPs into a Bloom Filter
    pub fn test_bloom_mem(blacklist_size: usize, false_positive_rate: f64) {
        println!("========================================");
        println!("Bloom Filter Insertion Memory Test");
        println!(
            "Inserting {} IPs with FPR {:.2}...",
            blacklist_size, false_positive_rate
        );

        let start = Instant::now();

        // Insert generated IPs directly into the Bloom Filter
        let mut filter = PacketFilter::new(blacklist_size, false_positive_rate);
        for _ in 0..blacklist_size {
            let ip = random_ip();
            filter.add_blacklisted_ip(ip);
        }

        let elapsed = start.elapsed();
        println!("Insertion completed in {:.3}s", elapsed.as_secs_f64());
        println!("========================================");
    }
}
