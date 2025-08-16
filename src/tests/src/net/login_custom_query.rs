use ferrumc_net::packets::incoming::custom_query_answer::CustomQueryAnswerPacket;
use ferrumc_net::packets::outgoing::custom_query::CustomQueryPacket;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Cursor, Write};

#[test]
fn decode_custom_query_answer() {
    let mut bytes = Vec::new();
    VarInt::from(5)
        .encode(&mut bytes, &NetEncodeOpts::None)
        .unwrap();
    true.encode(&mut bytes, &NetEncodeOpts::None).unwrap();
    VarInt::from(3)
        .encode(&mut bytes, &NetEncodeOpts::None)
        .unwrap();
    bytes.extend_from_slice(&[1, 2, 3]);
    let mut cursor = Cursor::new(bytes);
    let pkt = CustomQueryAnswerPacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(pkt.transaction_id, VarInt::from(5));
    match pkt.data {
        PrefixedOptional::Some(ref v) => {
            assert_eq!(v.length, VarInt::from(3));
            assert_eq!(v.data, vec![1, 2, 3]);
        }
        PrefixedOptional::None => panic!("expected data"),
    }
}

#[test]
fn encode_custom_query_packet() {
    let pkt = CustomQueryPacket::new(VarInt::from(7), "minecraft:brand", vec![1, 2, 3]);
    let mut buf = Vec::new();
    pkt.encode(&mut buf, &NetEncodeOpts::WithLength).unwrap();
    assert!(!buf.is_empty());
}
