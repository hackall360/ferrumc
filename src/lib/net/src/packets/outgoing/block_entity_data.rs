use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{network_position::NetworkPosition, var_int::VarInt};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "block_entity_data", state = "play")]
pub struct BlockEntityData {
    pub location: NetworkPosition,
    pub entity_type: VarInt,
    pub nbt: Vec<u8>,
}
