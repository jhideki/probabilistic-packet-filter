mod bloom_filter;
mod ddos;
mod packet_filter;

use ddos::ddos_test;

fn main() {
    ddos_test();
}

