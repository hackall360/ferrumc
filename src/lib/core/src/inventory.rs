use bevy_ecs::prelude::Component;
use ferrumc_world::block_id::BlockId;

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: BlockId,
    pub count: u8,
}

impl ItemStack {
    pub fn new(item: BlockId, count: u8) -> Self {
        Self { item, count }
    }
}

pub type Slot = Option<ItemStack>;

#[derive(Component, Debug, Clone, Default)]
pub struct Inventory {
    pub hotbar: [Slot; 9],
    pub main: [Slot; 27],
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_slot(&self, index: usize) -> Option<&Slot> {
        if index < self.main.len() {
            self.main.get(index)
        } else {
            let idx = index.checked_sub(self.main.len())?;
            self.hotbar.get(idx)
        }
    }

    pub fn all_slots(&self) -> Vec<Slot> {
        let mut slots = Vec::with_capacity(36);
        slots.extend(self.main.iter().cloned());
        slots.extend(self.hotbar.iter().cloned());
        slots
    }
}
