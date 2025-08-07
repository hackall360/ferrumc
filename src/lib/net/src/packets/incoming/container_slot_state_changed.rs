use crate::packets::slot::Slot;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "container_slot_state_changed", state = "play")]
pub struct ContainerSlotStateChanged {
    pub slot: i16,
    pub item: Slot,
}
