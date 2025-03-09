use crate::packet_filter::{PacketFilter, PacketFilterHashSet};
use crate::utils::random_ip;
use rand::seq::{IndexedRandom, SliceRandom};
use std::net::Ipv4Addr;
use std::time::Instant;

pub struct Ddos {
    incoming_packets: Vec<Ipv4Addr>,
    blacklist_ips: Vec<Ipv4Addr>,
    test_packet_count: usize,
}

impl Ddos {
    pub fn new(blacklist_size: i32, test_packet_count: usize, attack_ratio: f64) -> Self {
        let blacklist_ips: Vec<Ipv4Addr> = (0..blacklist_size).map(|_| random_ip()).collect();

        // Generate Incoming Packets
        println!("Generating {} incoming packets...", test_packet_count);
        let mut incoming_packets = Vec::new();
        let mut rng = rand::rng();

        let attack_packets: Vec<Ipv4Addr> = blacklist_ips
            .choose_multiple(&mut rng, (test_packet_count as f64 * attack_ratio) as usize)
            .cloned()
            .collect();

        let normal_packets: Vec<Ipv4Addr> = (0..test_packet_count - attack_packets.len())
            .map(|_| random_ip())
            .collect();

        incoming_packets.extend(attack_packets);
        incoming_packets.extend(normal_packets);
        incoming_packets.shuffle(&mut rng);

        Self {
            incoming_packets,
            blacklist_ips,
            test_packet_count,
        }
    }

    pub fn test_hashset_mem(&self) {
        println!("========================================");
        println!("Running ddos mem test with hashset implementation");
        let mut packet_filter_hash = PacketFilterHashSet::new();

        // Init packet filters with blacklisted ips`
        for ip in self.blacklist_ips.clone() {
            packet_filter_hash.add_blacklisted_ip(ip.clone());
        }
        // Start Benchmarking
        println!("Running DDoS filtering benchmark...");
        let mut total_checked = 0;
        let start_time = Instant::now();

        for ip in &self.incoming_packets {
            let _detected = packet_filter_hash.is_malicious(*ip);
            total_checked += 1;
        }

        let elapsed = start_time.elapsed();
        let throughput = total_checked as f64 / elapsed.as_secs_f64();

        // Results
        println!("=== DDoS Packet Filtering Performance ===");
        println!("Total Packets Checked: {}", total_checked);
        println!("Elapsed Time: {:.2} sec", elapsed.as_secs_f64());
        println!("Throughput: {:.2} packets/sec", throughput);
        println!("========================================");
    }

    pub fn test_bloom(&self) {
        println!("========================================");
        println!("Running ddos test with bloom filter implementation");
        let mut packet_filter = PacketFilter::new(self.blacklist_ips.len(), 0.1);
        let mut packet_filter_hash = PacketFilterHashSet::new();

        // Init packet filters with blacklisted ips`
        for ip in &self.blacklist_ips {
            packet_filter.add_blacklisted_ip(*ip);
            packet_filter_hash.add_blacklisted_ip(ip.clone());
        }
        // Start Benchmarking
        println!("Running DDoS filtering benchmark...");
        let mut false_positives = 0;
        let mut false_negatives = 0;
        let mut total_checked = 0;
        let start_time = Instant::now();

        for ip in &self.incoming_packets {
            let detected = packet_filter.is_malicious(*ip);
            let actual = packet_filter_hash.is_malicious(*ip);

            if detected && !actual {
                false_positives += 1;
            }
            if !detected && actual {
                false_negatives += 1;
            }

            total_checked += 1;
        }

        let elapsed = start_time.elapsed();
        let throughput = total_checked as f64 / elapsed.as_secs_f64();

        // Results
        println!("=== DDoS Packet Filtering Performance ===");
        println!("Total Packets Checked: {}", total_checked);
        println!("Throughput: {:.2} packets/sec", throughput);
        println!(
            "False Positives: {} ({:.2}%)",
            false_positives,
            (false_positives as f64 / total_checked as f64) * 100.0
        );
        println!(
            "False Negatives: {} ({:.2}%)",
            false_negatives,
            (false_negatives as f64 / total_checked as f64) * 100.0
        );
        println!("========================================");
    }

    pub fn test_bloom_mem(&self) {
        println!("========================================");
        println!("Running ddos mem test with bloom filter implementation");
        let mut packet_filter = PacketFilter::new(self.blacklist_ips.len(), 0.5);

        // Init packet filters with blacklisted ips`
        for ip in &self.blacklist_ips {
            packet_filter.add_blacklisted_ip(*ip);
        }
        // Start Benchmarking
        println!("Running DDoS filtering benchmark...");
        let mut total_checked = 0;
        let start_time = Instant::now();

        for ip in &self.incoming_packets {
            let _detected = packet_filter.is_malicious(*ip);
            total_checked += 1;
        }

        let elapsed = start_time.elapsed();
        let throughput = total_checked as f64 / elapsed.as_secs_f64();

        // Results
        println!("=== DDoS Packet Filtering Performance ===");
        println!("Total Packets Checked: {}", total_checked);
        println!("Elapsed Time: {:.2} sec", elapsed.as_secs_f64());
        println!("Throughput: {:.2} packets/sec", throughput);
        println!("========================================");
    }
}
