use std::collections::HashSet;
use std::net::Ipv4Addr;

use crate::bloom_filter::BloomFilter;
// Packet Filter Struct
pub struct PacketFilter {
    bloom_filter: BloomFilter,
}

impl PacketFilter {
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bloom_filter: BloomFilter::new(size, hash_count),
        }
    }

    pub fn add_blacklisted_ip(&mut self, ip: Ipv4Addr) {
        self.bloom_filter.insert(&ip);
    }

    pub fn is_malicious(&self, ip: Ipv4Addr) -> bool {
        self.bloom_filter.contains(&ip)
    }
}

pub struct PacketFilterHashSet {
    blocked_ips: HashSet<Ipv4Addr>,
}

impl PacketFilterHashSet {
    pub fn new() -> Self {
        Self {
            blocked_ips: HashSet::new(),
        }
    }
    pub fn add_blacklisted_ip(&mut self, ip: Ipv4Addr) {
        self.blocked_ips.insert(ip);
    }

    pub fn is_malicious(&self, ip: Ipv4Addr) -> bool {
        self.blocked_ips.contains(&ip)
    }
}
