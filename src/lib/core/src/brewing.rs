use crate::inventory::{Inventory, ItemStack};
use bevy_ecs::prelude::{Component, Query};

#[derive(Component, Debug, Clone, Default)]
pub struct BrewingStand {
    pub brew_time: i16,
    pub brew_time_total: i16,
}

/// Simple brewing stand system that moves an item from slot 0 to slot 3 after brewing.
pub fn brewing_tick(mut query: Query<(&mut BrewingStand, &mut Inventory)>) {
    for (mut stand, mut inv) in &mut query {
        if stand.brew_time > 0 {
            stand.brew_time -= 1;
            if stand.brew_time <= 0 {
                if let Some(slot0) = inv.get_slot_mut(0) {
                    if let Some(input) = slot0.take() {
                        inv.set_slot(
                            3,
                            Some(ItemStack { count: input.count, ..input }),
                        );
                    }
                }
            }
        }
    }
}
