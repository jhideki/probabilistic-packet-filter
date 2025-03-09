mod bloom_filter;
mod ddos;
mod packet_filter;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use ddos::Ddos;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    test: Command,
    #[arg(short, long)]
    bloom: bool,
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
            let ddos = Ddos::new(100_000, 1_000_000, 0.5);
            if args.bloom {
                ddos.test_bloom();
            }
        }
        Command::DdosPerformance => {
            let ddos = Ddos::new(100_000, 1_000_000, 0.5);
            if args.bloom {
                ddos.test_bloom_mem();
            } else {
                ddos.test_hashset_mem();
            }
        }
    }
}
