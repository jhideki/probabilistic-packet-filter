use bloomfilter::Bloom;
use std::collections::HashSet;
use std::net::Ipv4Addr;

/// Packet filter using a Bloom filter
pub struct PacketFilter {
    bloom_filter: Bloom<Ipv4Addr>,
}

impl PacketFilter {
    /// Create a new Bloom filter with an expected capacity and false positive rate
    pub fn new(expected_entries: usize, false_positive_rate: f64) -> Self {
        Self {
            bloom_filter: Bloom::new_for_fp_rate(expected_entries, false_positive_rate)
                .expect("Error creating bloom filter"),
        }
    }

    /// Add an IP to the Bloom filter blacklist
    pub fn add_blacklisted_ip(&mut self, ip: Ipv4Addr) {
        self.bloom_filter.set(&ip);
    }

    /// Check if an IP is potentially malicious
    pub fn is_malicious(&self, ip: Ipv4Addr) -> bool {
        self.bloom_filter.check(&ip) // True means possibly in set, false means definitely not
    }
}

/// Packet filter using a HashSet for exact membership checking
pub struct PacketFilterHashSet {
    blocked_ips: HashSet<Ipv4Addr>,
}

impl PacketFilterHashSet {
    /// Create a new HashSet-based packet filter
    pub fn new() -> Self {
        Self {
            blocked_ips: HashSet::new(),
        }
    }

    /// Add an IP to the blacklist
    pub fn add_blacklisted_ip(&mut self, ip: Ipv4Addr) {
        self.blocked_ips.insert(ip);
    }

    /// Check if an IP is malicious (exact match)
    pub fn is_malicious(&self, ip: Ipv4Addr) -> bool {
        self.blocked_ips.contains(&ip)
    }
}
