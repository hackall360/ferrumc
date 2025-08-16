use ferrumc_net::packets::incoming::legacy_server_list_ping::LegacyServerListPingPacket;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use std::io::Cursor;

#[test]
fn decode_legacy_server_list_ping() {
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let _pkt = LegacyServerListPingPacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
}
