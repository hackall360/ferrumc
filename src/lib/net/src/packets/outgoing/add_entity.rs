use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "add_entity", state = "play")]
pub struct AddEntityPacket;
