use bloomfilter::Bloom;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BloomWrapper {
    bloom: Arc<Mutex<Bloom<String>>>,
}

impl BloomWrapper {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let bloom = Bloom::new_for_fp_rate(expected_items, false_positive_rate)
            .expect("Failed to create bloom filter struct.");
        Self {
            bloom: Arc::new(Mutex::new(bloom)),
        }
    }

    pub fn insert(&mut self, item: &str) {
        let mut b = self.bloom.lock().unwrap();
        b.set(&item.to_string());
    }

    pub fn contains(&self, item: &str) -> bool {
        let b = self.bloom.lock().unwrap();
        b.check(&item.to_string())
    }
}
