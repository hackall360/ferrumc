use crate::packets::slot::Slot;
use ferrumc_core::inventory::Inventory;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "container_set_content", state = "play")]
pub struct ContainerSetContentPacket {
    pub container_id: u8,
    pub state_id: i32,
    pub slots: LengthPrefixedVec<Slot>,
}

impl ContainerSetContentPacket {
    pub fn from_inventory(inv: &Inventory) -> Self {
        let slot_vec: Vec<Slot> = inv
            .all_slots()
            .iter()
            .map(|s| Slot::from_stack(s.as_ref()))
            .collect();
        Self {
            container_id: 0,
            state_id: 0,
            slots: LengthPrefixedVec::new(slot_vec),
        }
    }
}
