use bloomfilter::Bloom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1_000_000)
    } else {
        1_000_000
    };

    let fp_rate: f64 = if args.len() > 2 {
        args[2].parse().unwrap_or(0.01)
    } else {
        0.01
    };

    let mut rng = StdRng::seed_from_u64(42);
    let mut bloom = Bloom::new_for_fp_rate(n, fp_rate).expect("Failed to create bloom filter");

    let start = Instant::now();

    for _ in 0..n {
        let val: u64 = rng.gen();
        bloom.set(&val);
    }

    let duration = start.elapsed();
    println!(
        "BloomFilter inserted {} items in {:?} with false positive rate {}",
        n, duration, fp_rate
    );

    println!("Sleeping to allow memory measurement... (press Ctrl+C to exit)");
    loop {}
}
