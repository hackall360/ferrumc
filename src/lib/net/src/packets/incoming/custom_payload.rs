use ferrumc_macros::{NetDecode, packet};

/// Represents a plugin channel message sent from the client.
#[derive(Debug, NetDecode)]
#[packet(packet_id = "custom_payload", state = "play")]
pub struct CustomPayloadPacket {
    /// Namespaced plugin channel identifier (e.g. `minecraft:brand`).
    pub channel: String,
    /// Raw bytes carried by the payload. Contains the remaining packet data.
    pub data: Vec<u8>,
}
