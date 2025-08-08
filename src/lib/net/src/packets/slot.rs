use ferrumc_core::inventory::ItemStack;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, NetDecode, Clone, Debug, Default)]
pub struct ItemData {
    pub item_id: VarInt,
    pub count: u8,
}

impl From<&ItemStack> for ItemData {
    fn from(stack: &ItemStack) -> Self {
        Self {
            item_id: VarInt::new(stack.item.0 as i32),
            count: stack.count,
        }
    }
}

#[derive(NetEncode, NetDecode)]
pub struct Slot {
    pub item: PrefixedOptional<ItemData>,
}

impl Slot {
    pub fn from_stack(stack: Option<&ItemStack>) -> Self {
        match stack {
            Some(s) => Slot {
                item: PrefixedOptional::Some(ItemData::from(s)),
            },
            None => Slot {
                item: PrefixedOptional::None,
            },
        }
    }
}

impl Default for Slot {
    fn default() -> Self {
        Slot { item: PrefixedOptional::None }
    }
}

impl std::fmt::Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            PrefixedOptional::Some(data) => f.debug_struct("Slot").field("item", data).finish(),
            PrefixedOptional::None => f.debug_struct("Slot").field("item", &"None").finish(),
        }
    }
}

impl Clone for Slot {
    fn clone(&self) -> Self {
        match &self.item {
            PrefixedOptional::Some(data) => {
                Slot { item: PrefixedOptional::Some(data.clone()) }
            }
            PrefixedOptional::None => Slot { item: PrefixedOptional::None },
        }
    }
}
