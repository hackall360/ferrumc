use bevy_ecs::prelude::Component;
use ferrumc_world::{block_id::BlockId, recipes::RECIPES};
use ferrumc_storage::player_data::{InventoryData, ItemStackData};

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

    /// Simple smithing operation: combines base and addition into output.
    pub fn smith(&mut self, base: usize, addition: usize, output: usize) -> bool {
        let (Some(base_slot), Some(add_slot)) = (
            self.get_slot_mut(base),
            self.get_slot_mut(addition),
        ) else {
            return false;
        };
        let (Some(base_stack), Some(add_stack)) = (base_slot.as_mut(), add_slot.as_mut()) else {
            return false;
        };

        base_stack.count = base_stack.count.saturating_sub(1);
        add_stack.count = add_stack.count.saturating_sub(1);
        if base_stack.count == 0 {
            *base_slot = None;
        }
        if add_stack.count == 0 {
            *add_slot = None;
        }

        let mut result = base_stack.clone();
        result.count = 1;
        result.nbt.get_or_insert(vec![]).push(1);
        self.set_slot(output, Some(result));
        true
    }

    /// Placeholder brewing operation using ingredient and potion slots.
    pub fn brew(&mut self, ingredient: usize, potion_slots: [usize; 3]) -> bool {
        let Some(ing_slot) = self.get_slot_mut(ingredient) else {
            return false;
        };
        let Some(ing_stack) = ing_slot.as_mut() else {
            return false;
        };
        let mut processed = false;
        for &idx in potion_slots.iter() {
            if let Some(Some(potion)) = self.get_slot_mut(idx).map(|s| s.as_mut()) {
                potion.nbt.get_or_insert(vec![]).push(2);
                processed = true;
            }
        }
        if processed {
            ing_stack.count = ing_stack.count.saturating_sub(1);
            if ing_stack.count == 0 {
                *ing_slot = None;
            }
        }
        processed
    }

    /// Basic enchanting: consume lapis and add marker to item.
    pub fn enchant(&mut self, item: usize, lapis: usize, output: usize) -> bool {
        let (Some(item_slot), Some(lapis_slot)) =
            (self.get_slot_mut(item), self.get_slot_mut(lapis))
        else {
            return false;
        };
        let (Some(item_stack), Some(lapis_stack)) =
            (item_slot.as_mut(), lapis_slot.as_mut())
        else {
            return false;
        };
        lapis_stack.count = lapis_stack.count.saturating_sub(1);
        if lapis_stack.count == 0 {
            *lapis_slot = None;
        }
        let mut result = item_stack.clone();
        result.nbt.get_or_insert(vec![]).push(3);
        self.set_slot(output, Some(result));
        item_stack.count = item_stack.count.saturating_sub(1);
        if item_stack.count == 0 {
            *item_slot = None;
        }
        true
    }

    /// Simple anvil operation that merges two stacks.
    pub fn anvil(&mut self, left: usize, right: usize, output: usize) -> bool {
        let (Some(l_slot), Some(r_slot)) =
            (self.get_slot_mut(left), self.get_slot_mut(right))
        else {
            return false;
        };
        let (Some(left_stack), Some(right_stack)) = (l_slot.as_mut(), r_slot.as_mut()) else {
            return false;
        };
        if left_stack.item != right_stack.item {
            return false;
        }
        let mut result = left_stack.clone();
        result.count = left_stack
            .count
            .saturating_add(right_stack.count)
            .min(left_stack.max_stack_size);
        *l_slot = None;
        *r_slot = None;
        self.set_slot(output, Some(result));
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct CraftingGrid {
    pub slots: [Slot; 9],
}

impl CraftingGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_slot(&mut self, index: usize, slot: Slot) {
        if index < self.slots.len() {
            self.slots[index] = slot;
        }
    }

    /// Returns true if the current grid arrangement matches any known recipe.
    pub fn is_valid(&self) -> bool {
        RECIPES.iter().any(|recipe| {
            if recipe.pattern.len() != self.slots.len() {
                return false;
            }
            recipe.pattern.iter().enumerate().all(|(i, r)| match r {
                Some(expected) => self.slots[i]
                    .as_ref()
                    .map(|s| s.item)
                    .filter(|id| id == expected)
                    .is_some(),
                None => self.slots[i].is_none(),
            })
        })
    }

    pub fn craft(&mut self) -> Option<ItemStack> {
        for recipe in RECIPES.iter() {
            if recipe.pattern.len() != self.slots.len() {
                continue;
            }
            let matches = recipe
                .pattern
                .iter()
                .enumerate()
                .all(|(i, r)| match r {
                    Some(expected) => self.slots[i]
                        .as_ref()
                        .map(|s| s.item)
                        .filter(|id| id == expected)
                        .is_some(),
                    None => self.slots[i].is_none(),
                });
            if matches {
                for (i, r) in recipe.pattern.iter().enumerate() {
                    if r.is_some() {
                        if let Some(slot) = self.slots.get_mut(i) {
                            if let Some(stack) = slot {
                                if stack.count > 0 {
                                    stack.count -= 1;
                                }
                                if stack.count == 0 {
                                    *slot = None;
                                }
                            }
                        }
                    }
                }
                let (id, count) = recipe.output;
                return Some(ItemStack::new(id, count, 64, None));
            }
        }
        None
    }
}

impl From<&ItemStack> for ItemStackData {
    fn from(item: &ItemStack) -> Self {
        Self {
            item: item.item.0,
            count: item.count,
            max_stack_size: item.max_stack_size,
            nbt: item.nbt.clone(),
        }
    }
}

impl From<&ItemStackData> for ItemStack {
    fn from(data: &ItemStackData) -> Self {
        ItemStack {
            item: BlockId(data.item),
            count: data.count,
            max_stack_size: data.max_stack_size,
            nbt: data.nbt.clone(),
        }
    }
}

impl From<&Inventory> for InventoryData {
    fn from(inv: &Inventory) -> Self {
        let hotbar = inv
            .hotbar
            .iter()
            .map(|s| s.as_ref().map(ItemStackData::from))
            .collect();
        let main = inv
            .main
            .iter()
            .map(|s| s.as_ref().map(ItemStackData::from))
            .collect();
        let equipment = inv
            .equipment
            .iter()
            .map(|s| s.as_ref().map(ItemStackData::from))
            .collect();
        let offhand = inv.offhand.as_ref().map(ItemStackData::from);
        InventoryData {
            hotbar,
            main,
            equipment,
            offhand,
        }
    }
}

impl From<&InventoryData> for Inventory {
    fn from(data: &InventoryData) -> Self {
        let mut inv = Inventory::default();
        for (i, slot) in data.hotbar.iter().enumerate() {
            inv.hotbar[i] = slot.as_ref().map(ItemStack::from);
        }
        for (i, slot) in data.main.iter().enumerate() {
            inv.main[i] = slot.as_ref().map(ItemStack::from);
        }
        for (i, slot) in data.equipment.iter().enumerate() {
            inv.equipment[i] = slot.as_ref().map(ItemStack::from);
        }
        inv.offhand = data.offhand.as_ref().map(ItemStack::from);
        inv
    }
}
