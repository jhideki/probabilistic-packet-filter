use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1_000_000)
    } else {
        1_000_000
    };

    let mut rng = StdRng::seed_from_u64(42);
    let mut set = HashSet::with_capacity(n);

    let start = Instant::now();

    for _ in 0..n {
        let val: u64 = rng.gen();
        set.insert(val);
    }

    let duration = start.elapsed();
    println!("HashSet inserted {} items in {:?}", n, duration);

    println!("Sleeping to allow memory measurement... (press Ctrl+C to exit)");
    loop {}
}
