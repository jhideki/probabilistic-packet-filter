use crate::bloom::BloomWrapper;
use std::sync::Arc;

pub struct PacketFilter {
    pub bloom: Arc<BloomWrapper>,
}

impl PacketFilter {
    pub fn new(bloom: Arc<BloomWrapper>) -> Self {
        Self { bloom }
    }
}
