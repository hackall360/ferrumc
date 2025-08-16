use crate::inventory::{Inventory, ItemStack};
use bevy_ecs::prelude::{Component, Query};

#[derive(Component, Debug, Clone, Default)]
pub struct BrewingStand {
    pub brew_time: i16,
    pub brew_time_total: i16,
    pub fuel: i16,
}

fn blaze_fuel_value(stack: &ItemStack) -> i16 {
    stack
        .item
        .to_block_data()
        .map(|data| if data.name.contains("blaze_powder") { 20 } else { 0 })
        .unwrap_or(0)
}

fn brew_result(potion: &ItemStack, _ingredient: &ItemStack) -> ItemStack {
    let mut out = potion.clone();
    out.nbt.get_or_insert(vec![]).push(4);
    out
}

/// Brewing stand tick handling fuel and basic potion transformation.
pub fn brewing_tick(mut query: Query<(&mut BrewingStand, &mut Inventory)>) {
    for (mut stand, mut inv) in &mut query {
        // Load fuel if needed
        if stand.fuel <= 0 {
            if let Some(slot) = inv.get_slot_mut(4) {
                if let Some(fuel) = slot.as_mut() {
                    let value = blaze_fuel_value(fuel);
                    if value > 0 {
                        stand.fuel = value;
                        fuel.count = fuel.count.saturating_sub(1);
                        if fuel.count == 0 {
                            *slot = None;
                        }
                    }
                }
            }
        }

        if stand.brew_time <= 0 {
            if stand.fuel > 0 {
                let ingredient_present = inv.get_slot(3).and_then(|s| s.as_ref()).is_some();
                let potion_present = (0..3).any(|i| inv.get_slot(i).and_then(|s| s.as_ref()).is_some());
                if ingredient_present && potion_present {
                    stand.brew_time = stand.brew_time_total;
                    stand.fuel -= 1;
                }
            }
        } else {
            stand.brew_time -= 1;
            if stand.brew_time <= 0 {
                if let Some(ing_slot) = inv.get_slot_mut(3) {
                    if let Some(ingredient) = ing_slot.take() {
                        for i in 0..3 {
                            if let Some(slot) = inv.get_slot_mut(i) {
                                if let Some(potion) = slot.as_mut() {
                                    *potion = brew_result(potion, &ingredient);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

