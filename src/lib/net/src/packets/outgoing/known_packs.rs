use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "known_packs", state = "login")]
pub struct ClientboundKnownPacks<'a> {
    pub packs: LengthPrefixedVec<KnownPack<'a>>,
}

#[derive(NetEncode)]
pub struct KnownPack<'a> {
    pub namespace: &'a str,
    pub id: &'a str,
    pub version: &'a str,
}
