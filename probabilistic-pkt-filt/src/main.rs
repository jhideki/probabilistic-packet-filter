mod bloom_filter;
mod ddos;
mod packet_filter;
mod utils;

use clap::Parser;
use ddos::Ddos;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let ddos = Ddos::new(100_000, 1_000_000, 0.5);
}
