use crate::inventory::{Inventory, ItemStack};
use bevy_ecs::prelude::{Component, Query};
use ferrumc_world::block_id::{BlockData, BlockId};

#[derive(Component, Debug, Clone, Default)]
pub struct Furnace {
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
    pub fuel_time: i16,
    pub xp: f32,
}

/// Helper to determine how long a fuel item should burn for.
fn fuel_value(stack: &ItemStack) -> i16 {
    stack
        .item
        .to_block_data()
        .map(|data| {
            let name = data.name.as_str();
            if name.contains("coal") {
                1600
            } else if name.contains("log") || name.contains("planks") {
                300
            } else {
                0
            }
        })
        .unwrap_or(0)
}

/// Determines the smelting output and experience for a given input item.
fn smelting_result(input: &ItemStack) -> Option<(ItemStack, f32)> {
    let data = input.item.to_block_data()?;
    let name = data.name.as_str();
    let (out_name, xp) = if name.contains("iron_ore") {
        ("minecraft:iron_ingot", 0.7)
    } else if name.contains("gold_ore") {
        ("minecraft:gold_ingot", 1.0)
    } else if name.contains("sand") {
        ("minecraft:glass", 0.1)
    } else {
        return None;
    };
    let out_block = BlockData {
        name: out_name.to_string(),
        properties: None,
    };
    let out_id = BlockId::from_block_data(&out_block);
    Some((ItemStack::new(out_id, 1, 64, None), xp))
}

/// Furnace smelting system implementing fuel consumption, output conversion and XP gain.
pub fn furnace_tick(mut query: Query<(&mut Furnace, &mut Inventory)>) {
    for (mut furnace, mut inv) in &mut query {
        // Refill fuel if necessary
        if furnace.burn_time <= 0 {
            if let Some(slot1) = inv.get_slot_mut(1) {
                if let Some(fuel) = slot1.as_mut() {
                    let value = fuel_value(fuel);
                    if value > 0 {
                        furnace.burn_time = value;
                        furnace.fuel_time = value;
                        fuel.count = fuel.count.saturating_sub(1);
                        if fuel.count == 0 {
                            *slot1 = None;
                        }
                    }
                }
            }
        }

        if furnace.burn_time > 0 {
            furnace.burn_time -= 1;

            if let Some(slot0) = inv.get_slot_mut(0) {
                if let Some(input) = slot0.as_mut() {
                    furnace.cook_time += 1;
                    if furnace.cook_time >= furnace.cook_time_total {
                        if let Some((result, xp)) = smelting_result(input) {
                            furnace.cook_time = 0;
                            input.count = input.count.saturating_sub(1);
                            if input.count == 0 {
                                *slot0 = None;
                            }

                            // Place result in output slot 2
                            if let Some(existing) = inv.get_slot_mut(2) {
                                match existing {
                                    Some(out_stack) if out_stack.item == result.item => {
                                        out_stack.count = out_stack
                                            .count
                                            .saturating_add(result.count)
                                            .min(out_stack.max_stack_size);
                                    }
                                    Some(_) => {}
                                    None => {
                                        *existing = Some(result);
                                    }
                                }
                            }

                            furnace.xp += xp;
                        }
                    }
                }
            }
        } else {
            furnace.cook_time = 0;
        }
    }
}

