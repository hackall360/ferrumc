use ferrumc_macros::{NetDecode, packet};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode, Debug)]
#[packet(packet_id = "block_entity_tag_query", state = "play")]
pub struct BlockEntityTagQuery {
    pub transaction_id: VarInt,
    pub location: NetworkPosition,
}
