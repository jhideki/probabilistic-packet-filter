use crate::packet_filter::{PacketFilter, PacketFilterHashSet};
use crate::utils::random_ip;
use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::time::Instant;

pub struct Ddos {
    packet_filter: PacketFilter,
    packet_filter_hash: PacketFilterHashSet,
    incoming_packets: Vec<Ipv4Addr>,
}

impl Ddos {
    pub fn new(blacklist_size: i32, test_packet_count: usize, attack_ratio: f64) -> Self {
        let mut packet_filter = PacketFilter::new(test_packet_count, 3);
        let mut packet_filter_hash = PacketFilterHashSet::new();

        let blacklist_ips: Vec<Ipv4Addr> = (0..blacklist_size).map(|_| random_ip()).collect();

        // Init packet filters with blacklisted ips`
        for ip in blacklist_ips.clone() {
            packet_filter.add_blacklisted_ip(ip);
            packet_filter_hash.add_blacklisted_ip(ip.clone());
        }
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
            packet_filter,
            packet_filter_hash,
            incoming_packets,
        }
    }

    pub fn test_hash_set(&self) {
        let packet_filter_hash = &self.packet_filter_hash;
        // Start Benchmarking
        println!("Running DDoS filtering benchmark...");
        let mut false_positives = 0;
        let mut false_negatives = 0;
        let mut total_checked = 0;
        let start_time = Instant::now();

        for ip in &self.incoming_packets {
            //Run twice
            let detected = packet_filter_hash.is_malicious(*ip);
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
        println!("Elapsed Time: {:.2} sec", elapsed.as_secs_f64());
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

    pub fn test_bloom(&self) {
        let packet_filter = &self.packet_filter;
        let packet_filter_hash = &self.packet_filter_hash;
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
        println!("Elapsed Time: {:.2} sec", elapsed.as_secs_f64());
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
}

pub fn ddos_bloom() {
    let blacklist_size = 100_000;
    let test_packet_count = 1_000_000;
    let attack_ratio = 0.5; // 50% attack packets

    //Bloom filter impl
    let mut packet_filter = PacketFilter::new(1_000_000, 3);
    //Hashset impl
    let mut packet_filter_hash = PacketFilterHashSet::new();

    // Generate Blacklist IPs
    println!("Generating {} malicious IPs...", blacklist_size);
    let blacklist_ips: Vec<Ipv4Addr> = (0..blacklist_size).map(|_| random_ip()).collect();

    // Init packet filters with blacklisted ips`
    for ip in blacklist_ips.clone() {
        packet_filter.add_blacklisted_ip(ip);
        packet_filter_hash.add_blacklisted_ip(ip.clone());
    }
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

    // Start Benchmarking
    println!("Running DDoS filtering benchmark...");
    let mut false_positives = 0;
    let mut false_negatives = 0;
    let mut total_checked = 0;
    let start_time = Instant::now();

    for ip in incoming_packets {
        let detected = packet_filter.is_malicious(ip);
        let actual = packet_filter_hash.is_malicious(ip);

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
    println!("Elapsed Time: {:.2} sec", elapsed.as_secs_f64());
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
