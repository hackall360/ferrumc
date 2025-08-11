use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

/// Sends a plugin channel message to the client.
#[derive(NetEncode, Clone)]
#[packet(packet_id = "custom_payload", state = "play")]
pub struct CustomPayloadPacket {
    /// Namespaced plugin channel identifier (e.g. `minecraft:register`).
    pub channel: String,
    /// Raw plugin channel data to forward.
    pub data: Vec<u8>,
}

impl CustomPayloadPacket {
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        Self { channel, data }
    }
}
