use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode)]
#[packet(packet_id = "chat_ack", state = "play")]
pub struct ChatAckPacket {
    pub count: VarInt,
}
