use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "custom_query_answer", state = "login")]
pub struct CustomQueryAnswerPacket {
    pub transaction_id: VarInt,
    pub data: PrefixedOptional<LengthPrefixedVec<u8>>,
}
