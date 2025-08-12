use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_text::TextComponent;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "chat_message", state = "play")]
pub struct OutgoingChatMessagePacket {
    pub message: TextComponent,
    pub signature: PrefixedOptional<String>,
}

impl OutgoingChatMessagePacket {
    pub fn new(message: TextComponent) -> Self {
        Self { message, signature: PrefixedOptional::None }
    }

    pub fn with_signature(message: TextComponent, signature: Option<String>) -> Self {
        Self { message, signature: PrefixedOptional::new(signature) }
    }
}
