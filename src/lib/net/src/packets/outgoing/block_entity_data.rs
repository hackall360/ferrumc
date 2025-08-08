use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "block_entity_data", state = "play")]
pub struct BlockEntityData {
    pub location: NetworkPosition,
    pub entity_type: VarInt,
    pub data: Vec<u8>,
}
