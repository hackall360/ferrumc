use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug)]
#[packet(packet_id = "chat_message", state = "play")]
pub struct IncomingChatMessagePacket {
    pub message: String,
}
