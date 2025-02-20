use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct ProbabilisticFilter {
    bit_array: Vec<bool>,
    size: usize,
}

impl ProbabilisticFilter {
    fn new(size: usize) -> Self {
        Self {
            bit_array: vec![false; size],
            size,
        }
    }

    fn hash<T: Hash>(&self, item: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        (hasher.finish() as usize) % self.size
    }

    fn insert<T: Hash>(&mut self, item: &T) {
        let index = self.hash(item);
        self.bit_array[index] = true;
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        let index = self.hash(item);
        self.bit_array[index]
    }
}

fn main() {
    let mut filter = ProbabilisticFilter::new(1000);
    
    filter.insert(&"packet_1");
    
    println!("Contains packet_1? {}", filter.contains(&"packet_1"));
    println!("Contains packet_2? {}", filter.contains(&"packet_2"));
}
    println!("Hello, world!");
}
