use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "chat_ack", state = "play")]
pub struct ChatAckPacket;
