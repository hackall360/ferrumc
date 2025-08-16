use ferrumc_macros::{packet, NetDecode};

#[derive(Debug, NetDecode)]
#[packet(packet_id = "legacy_server_list_ping", state = "handshake")]
pub struct LegacyServerListPingPacket;
