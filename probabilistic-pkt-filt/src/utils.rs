use crate::packet_filter::PacketFilter;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::time::Instant;

pub fn random_ip() -> Ipv4Addr {
    let mut rng = rand::rng();
    Ipv4Addr::new(
        rng.random_range(1..=255),
        rng.random_range(1..=255),
        rng.random_range(1..=255),
        rng.random_range(1..=255),
    )
}
