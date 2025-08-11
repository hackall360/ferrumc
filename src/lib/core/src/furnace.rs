use crate::inventory::{Inventory, ItemStack};
use bevy_ecs::prelude::{Component, Query};

#[derive(Component, Debug, Clone, Default)]
pub struct Furnace {
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
}

/// Simple furnace smelting system. This is a placeholder and only decrements timers.
pub fn furnace_tick(mut query: Query<(&mut Furnace, &mut Inventory)>) {
    for (mut furnace, mut inv) in &mut query {
        if furnace.burn_time > 0 {
            furnace.burn_time -= 1;
            furnace.cook_time += 1;
            if furnace.cook_time >= furnace.cook_time_total {
                furnace.cook_time = 0;
                // Very primitive smelting: move input slot 0 to output slot 2.
                if let Some(slot0) = inv.get_slot_mut(0) {
                    if let Some(input) = slot0.take() {
                        inv.set_slot(
                            2,
                            Some(ItemStack {
                                count: input.count,
                                ..input
                            }),
                        );
                    }
                }
            }
        }
    }
}
