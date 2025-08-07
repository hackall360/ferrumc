use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "known_packs", state = "login")]
pub struct ServerboundKnownPacks {
    pub packs: LengthPrefixedVec<KnownPack>,
}

#[derive(Debug, NetDecode, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
