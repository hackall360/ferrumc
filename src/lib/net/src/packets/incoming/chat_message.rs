use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode, Debug)]
#[packet(packet_id = "chat_message", state = "play")]
pub struct IncomingChatMessagePacket {
    pub message: String,
}

