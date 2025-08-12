use ferrumc_net::packets::incoming::{chat_ack::ChatAckPacket, chat_message::IncomingChatMessagePacket};
use ferrumc_net::packets::outgoing::chat_message::OutgoingChatMessagePacket;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;
use std::io::{Cursor, Write};

#[test]
fn encode_signed_outgoing_message() {
    let msg = TextComponent::from("hi");
    let pkt = OutgoingChatMessagePacket::with_signature(msg, Some("sig".into()));
    let mut buf = Vec::new();
    pkt.encode(&mut buf, &NetEncodeOpts::WithLength).unwrap();
    assert!(!buf.is_empty());
}

#[test]
fn decode_signed_incoming_message_and_ack() {
    let mut bytes = Vec::new();
    // message length + bytes
    VarInt::new(2).encode(&mut bytes, &NetEncodeOpts::None).unwrap();
    bytes.write_all(b"hi").unwrap();
    // signature present + value
    true.encode(&mut bytes, &NetEncodeOpts::None).unwrap();
    "sig".encode(&mut bytes, &NetEncodeOpts::None).unwrap();
    let mut cursor = Cursor::new(bytes);
    let pkt = IncomingChatMessagePacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(pkt.message, "hi");
    assert!(pkt.signature.is_some());

    let mut ack_bytes = Vec::new();
    VarInt::new(1).encode(&mut ack_bytes, &NetEncodeOpts::None).unwrap();
    let mut cursor = Cursor::new(ack_bytes);
    let ack = ChatAckPacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(ack.count.0, 1);
}
