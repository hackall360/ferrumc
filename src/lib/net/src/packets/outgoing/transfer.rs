use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "transfer", state = "handshake")]
pub struct TransferPacket {
    pub host: String,
    pub port: u16,
}

impl TransferPacket {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }
}
