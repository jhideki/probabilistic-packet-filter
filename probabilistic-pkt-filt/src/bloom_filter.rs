/*use bitvec::prelude::*;
use std::hash::{Hash, Hasher};
use twox_hash::xxhash3_64::Hasher; // Faster non-allocating hash function

pub struct BloomFilter {
    bit_array: BitVec, // Efficient bit storage
    size: usize,
    hash_count: usize,
}

impl BloomFilter {
    /// Create a new Bloom filter with a given size and number of hash functions
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bit_array: bitvec![0; size], // Initialize all bits to 0
            size,
            hash_count,
        }
    }

    /// Computes `hash_count` different hashes for an item
    fn hash<T: Hash>(&self, item: &T, i: usize) -> usize {
        let base_hash = Hasher(&bincode::serialize(item).unwrap()); // Hash item only once
        let new_hash = base_hash.wrapping_add(i.wrapping_mul(0x9e3779b9)); // Mix different hashes
        new_hash % self.size
    }

    /// Insert an item into the Bloom filter
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            self.bit_array.set(index, true);
        }
    }

    /// Check if an item is possibly in the set
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.hash_count {
            let index = self.hash(item, i);
            if !self.bit_array[index] {
                return false; // If any bit is 0, item is definitely not in the set
            }
        }
        true // Otherwise, item is probably in the set
    }
}
*/
