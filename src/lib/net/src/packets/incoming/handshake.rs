use ferrumc_macros::{NetDecode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "intention", state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[cfg(test)]
mod tests {
    use ferrumc_macros::NetDecode;
    use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
    use ferrumc_net_codec::net_types::var_int::VarInt;
    use std::io::Cursor;

    #[test]
    fn test_macro_decode() {
        #[derive(NetDecode, Default)]
        struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        let mut data = Cursor::new(vec![
            0xfb, 0x05, // protocol version 763 (Minecraft 1.20.1)
            0x09, b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't', // "localhost"
            0x63, 0xdd, // server port 25565
            0x01, // next state: status
        ]);

        let handshake = Handshake::decode(&mut data, &NetDecodeOpts::None).unwrap();
        // The protocol version for Minecraft 1.20.1 is 763; this test uses that value.
        assert_eq!(handshake.protocol_version, VarInt::new(763));
        assert_eq!(handshake.server_address, "localhost".to_string());
        assert_eq!(handshake.server_port, 25565);
        assert_eq!(handshake.next_state, VarInt::new(1));
    }
}
