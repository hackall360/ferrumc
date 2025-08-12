use bevy_ecs::prelude::Component;
use ferrumc_world::block_id::BlockId;

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: BlockId,
    pub count: u8,
    pub max_stack_size: u8,
    pub nbt: Option<Vec<u8>>,
}

impl ItemStack {
    pub fn new(item: BlockId, count: u8, max_stack_size: u8, nbt: Option<Vec<u8>>) -> Self {
        let count = count.min(max_stack_size);
        Self {
            item,
            count,
            max_stack_size,
            nbt,
        }
    }
}

pub type Slot = Option<ItemStack>;

/// Result of using an item via right click.
#[derive(Debug, PartialEq)]
pub enum ItemUseResult {
    Placed,
    Eaten,
    ShotBow,
    NoItem,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Inventory {
    pub hotbar: [Slot; 9],
    pub main: [Slot; 27],
    pub equipment: [Slot; 4],
    pub offhand: Slot,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_slot(&self, index: usize) -> Option<&Slot> {
        let main_len = self.main.len();
        let hotbar_len = self.hotbar.len();
        let equip_len = self.equipment.len();
        if index < main_len {
            Some(&self.main[index])
        } else if index < main_len + hotbar_len {
            Some(&self.hotbar[index - main_len])
        } else if index < main_len + hotbar_len + equip_len {
            Some(&self.equipment[index - main_len - hotbar_len])
        } else if index == main_len + hotbar_len + equip_len {
            Some(&self.offhand)
        } else {
            None
        }
    }

    pub fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        let main_len = self.main.len();
        let hotbar_len = self.hotbar.len();
        let equip_len = self.equipment.len();
        if index < main_len {
            Some(&mut self.main[index])
        } else if index < main_len + hotbar_len {
            Some(&mut self.hotbar[index - main_len])
        } else if index < main_len + hotbar_len + equip_len {
            Some(&mut self.equipment[index - main_len - hotbar_len])
        } else if index == main_len + hotbar_len + equip_len {
            Some(&mut self.offhand)
        } else {
            None
        }
    }

    pub fn set_slot(&mut self, index: usize, slot: Slot) {
        if let Some(s) = self.get_slot_mut(index) {
            *s = slot;
        }
    }

    pub fn all_slots(&self) -> Vec<Slot> {
        let mut slots = Vec::with_capacity(41);
        slots.extend(self.main.iter().cloned());
        slots.extend(self.hotbar.iter().cloned());
        slots.extend(self.equipment.iter().cloned());
        slots.push(self.offhand.clone());
        slots
    }

    /// Attempts to add an item to a slot, respecting max stack sizes.
    /// Returns any leftover items that couldn't fit.
    pub fn add_item_to_slot(&mut self, index: usize, mut item: ItemStack) -> Option<ItemStack> {
        if let Some(slot) = self.get_slot_mut(index) {
            match slot {
                Some(existing) if existing.item == item.item && existing.nbt == item.nbt => {
                    let available = existing.max_stack_size.saturating_sub(existing.count);
                    let to_add = item.count.min(available);
                    existing.count += to_add;
                    item.count -= to_add;
                    if item.count > 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Some(_) => Some(item),
                None => {
                    let count = item.count.min(item.max_stack_size);
                    item.count = count;
                    *slot = Some(item);
                    None
                }
            }
        } else {
            Some(item)
        }
    }

    /// Uses the item in the specified slot as if right-clicked.
    pub fn right_click_slot(&mut self, index: usize) -> ItemUseResult {
        if let Some(slot) = self.get_slot_mut(index) {
            if let Some(stack) = slot {
                if let Some(data) = stack.item.to_block_data() {
                    let name = data.name.as_str();
                    if name.contains("bow") {
                        return ItemUseResult::ShotBow;
                    }
                    stack.count = stack.count.saturating_sub(1);
                    if stack.count == 0 {
                        *slot = None;
                    }
                    return if name.contains("apple") || name.contains("bread") {
                        ItemUseResult::Eaten
                    } else {
                        ItemUseResult::Placed
                    };
                }
            }
        }
        ItemUseResult::NoItem
    }
}
