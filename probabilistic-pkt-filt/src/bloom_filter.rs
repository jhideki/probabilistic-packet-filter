use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bit_array: Vec<u8>, // Use u8 for better memory efficiency
    size: usize,
    hash_count: usize, // Number of hash functions
}

impl BloomFilter {
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bit_array: vec![0; (size + 7) / 8], // Use bit packing
            size,
            hash_count,
        }
    }

    fn hash<T: Hash>(&self, item: &T, i: usize) -> usize {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        item.hash(&mut hasher1);
        let hash1 = hasher1.finish() as usize;

        item.hash(&mut hasher2);
        let hash2 = hasher2.finish() as usize;

        (hash1.wrapping_add(i.wrapping_mul(hash2))) % self.size
    }

    /// Insert an item into the Bloom filter
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            self.set_bit(index);
        }
    }

    /// Check if an item is possibly in the set
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            if !self.get_bit(index) {
                return false;
            }
        }
        true
    }

    /// Set a bit in the bit array
    fn set_bit(&mut self, index: usize) {
        let byte_pos = index / 8;
        let bit_pos = index % 8;
        self.bit_array[byte_pos] |= 1 << bit_pos;
    }

    /// Get a bit from the bit array
    fn get_bit(&self, index: usize) -> bool {
        let byte_pos = index / 8;
        let bit_pos = index % 8;
        (self.bit_array[byte_pos] & (1 << bit_pos)) != 0
    }
}
