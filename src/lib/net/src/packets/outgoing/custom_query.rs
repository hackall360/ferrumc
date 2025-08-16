use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "custom_query", state = "login")]
pub struct CustomQueryPacket<'a> {
    pub transaction_id: VarInt,
    pub channel: &'a str,
    pub data: ByteArray,
}

impl<'a> CustomQueryPacket<'a> {
    pub fn new(transaction_id: VarInt, channel: &'a str, data: Vec<u8>) -> Self {
        Self {
            transaction_id,
            channel,
            data: ByteArray::new(data),
        }
    }
}
