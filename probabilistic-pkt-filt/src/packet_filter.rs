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

    pub fn add_blacklisted_ip(&mut self, ip: &str) {
        self.bloom_filter.insert(ip);
    }

    pub fn is_malicious(&self, ip: &str) -> bool {
        self.bloom_filter.contains(ip)
    }
}
