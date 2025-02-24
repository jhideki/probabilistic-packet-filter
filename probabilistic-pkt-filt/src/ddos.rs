use crate::packet_filter::PacketFilter;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

fn random_ip() -> String {
    let mut rng = rand::rng();
    format!(
        "{}.{}.{}.{}",
        rng.random_range(1..=255),
        rng.random_range(1..=255),
        rng.random_range(1..=255),
        rng.random_range(1..=255)
    )
}

pub fn ddos_test() {
    let blacklist_size = 100_000;
    let test_packet_count = 1_000_000;
    let attack_ratio = 0.5; // 50% attack packets

    let mut packet_filter = PacketFilter::new(1_000_000, 3);
    let mut true_blacklist = HashSet::new();

    // Generate Blacklist IPs
    println!("Generating {} malicious IPs...", blacklist_size);
    let blacklist_ips: Vec<String> = (0..blacklist_size).map(|_| random_ip()).collect();

    for ip in &blacklist_ips {
        packet_filter.add_blacklisted_ip(ip);
        true_blacklist.insert(ip.clone());
    }

    // Generate Incoming Packets
    println!("Generating {} incoming packets...", test_packet_count);
    let mut incoming_packets = Vec::new();
    let mut rng = rand::rng();

    let attack_packets: Vec<String> = blacklist_ips
        .choose_multiple(&mut rng, (test_packet_count as f64 * attack_ratio) as usize)
        .cloned()
        .collect();

    let normal_packets: Vec<String> = (0..test_packet_count - attack_packets.len())
        .map(|_| random_ip())
        .collect();

    incoming_packets.extend(attack_packets);
    incoming_packets.extend(normal_packets);
    incoming_packets.shuffle(&mut rng);

    // Start Benchmarking
    println!("Running DDoS filtering benchmark...");
    let start_time = Instant::now();

    let mut false_positives = 0;
    let mut false_negatives = 0;
    let mut total_checked = 0;

    for ip in incoming_packets {
        let detected = packet_filter.is_malicious(&ip);
        let actual = true_blacklist.contains(&ip);

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
