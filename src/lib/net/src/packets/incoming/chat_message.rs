use ferrumc_macros::{NetDecode, packet};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "chat_message", state = "play")]
pub struct IncomingChatMessagePacket {
    pub message: String,
    pub signature: PrefixedOptional<String>,
}
