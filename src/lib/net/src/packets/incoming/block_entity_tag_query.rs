use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::{network_position::NetworkPosition, var_int::VarInt};

#[derive(NetDecode)]
#[packet(packet_id = "block_entity_tag_query", state = "play")]
pub struct BlockEntityTagQuery {
    pub transaction_id: VarInt,
    pub location: NetworkPosition,
}
