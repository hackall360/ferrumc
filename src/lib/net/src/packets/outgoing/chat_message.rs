use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::TextComponent;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "chat_message", state = "play")]
pub struct OutgoingChatMessagePacket {
    pub message: TextComponent,
}

impl OutgoingChatMessagePacket {
    pub fn new(message: TextComponent) -> Self {
        Self { message }
    }
}

