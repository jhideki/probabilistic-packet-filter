mod bloom_filter;
mod ddos;
mod packet_filter;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use ddos::Ddos;

#[derive(Parser)]
#[command(name = "PacketFilter Benchmark")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Compare HashSet vs Bloom Filter for packet filtering", long_about = None)]
struct Args {
    #[arg(short, long)]
    test: Command,

    #[arg(short, long)]
    bloom: bool,

    #[arg(long, default_value_t = 1_000_000)]
    blacklist_size: usize,

    #[arg(long, default_value_t = 1_000_000)]
    packet_count: usize,

    #[arg(long, default_value_t = 0.5)]
    attack_ratio: f64,

    #[arg(long, default_value_t = 0.01)]
    false_positive_rate: f64,
}

#[derive(ValueEnum, Clone, Debug, Subcommand)]
enum Command {
    DdosAccuracy,
    DdosPerformance,
}

fn main() {
    let args = Args::parse();

    match args.test {
        Command::DdosAccuracy => {
            if args.bloom {
                println!("Bloom filter accuracy test not implemented in this version.");
            } else {
                println!("HashSet accuracy test not implemented in this version.");
            }
        }
        Command::DdosPerformance => {
            if args.bloom {
                Ddos::test_bloom_mem(args.blacklist_size, args.false_positive_rate);
            } else {
                Ddos::test_hashset_mem(args.blacklist_size);
            }
        }
    }
}
