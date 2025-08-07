use crate::packets::slot::Slot;
use ferrumc_core::inventory::ItemStack;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(packet_id = "container_set_slot", state = "play")]
pub struct ContainerSetSlotPacket {
    pub container_id: u8,
    pub state_id: i32,
    pub slot: i16,
    pub item: Slot,
}

impl ContainerSetSlotPacket {
    pub fn new(container_id: u8, state_id: i32, slot: i16, item: Option<&ItemStack>) -> Self {
        Self {
            container_id,
            state_id,
            slot,
            item: Slot::from_stack(item),
        }
    }
}
