use bevy_ecs::prelude::Component;
use ferrumc_world::chunk_format::Chunk;
use typename::TypeName;

use crate::{
    collisions::{
        bounds::CollisionBounds,
        world::{for_each_block_in_bounds, resolve_block_collisions},
    },
    transform::position::Position,
};

/// Base movement speed for an entity.
#[derive(TypeName, Component, Debug, Clone, Copy)]
pub struct Speed(pub f64);

impl Default for Speed {
    fn default() -> Self {
        Speed(0.1)
    }
}

#[derive(TypeName, Component, Debug, Clone, Copy, Default)]
pub struct Movement {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// Whether the entity is currently sneaking.
    pub sneaking: bool,
}

impl Movement {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            vx,
            vy,
            vz,
            sneaking: false,
        }
    }

    /// Apply gravity based on whether the entity is in a fluid.
    fn apply_gravity(&mut self, in_fluid: bool) {
        const GRAVITY: f64 = -0.08;
        const WATER_BUOYANCY: f64 = 0.04;
        if in_fluid {
            self.vy += WATER_BUOYANCY;
        } else {
            self.vy += GRAVITY;
        }
    }

    /// Apply drag depending on whether the entity is in fluid.
    fn apply_drag(&mut self, in_fluid: bool) {
        const AIR_DRAG: f64 = 0.98;
        const WATER_DRAG: f64 = 0.8;
        let drag = if in_fluid { WATER_DRAG } else { AIR_DRAG };
        self.vx *= drag;
        self.vy *= drag;
        self.vz *= drag;
    }

    /// Apply ground friction to horizontal movement when on the ground.
    fn apply_friction(&mut self, on_ground: bool) {
        const FRICTION: f64 = 0.91;
        if on_ground {
            self.vx *= FRICTION;
            self.vz *= FRICTION;
        }
    }

    /// Apply forces from water flow. Currently only adds upward flow.
    fn apply_water_flow(&mut self, in_fluid: bool) {
        if in_fluid {
            // Simple upward current to emulate swimming buoyancy.
            self.vy += 0.01;
        }
    }

    /// Tick movement, applying gravity, drag, friction and flow forces.
    pub fn tick(&mut self, position: &mut Position, bounds: &CollisionBounds, chunk: &Chunk) {
        let in_fluid = is_in_fluid(chunk, position, bounds);
        let climbing = is_touching_climbable(chunk, position, bounds);
        if !climbing {
            self.apply_gravity(in_fluid);
        } else {
            self.vy = self.vy.clamp(-0.15, 0.2);
        }
        self.apply_water_flow(in_fluid);
        self.apply_drag(in_fluid);
        if self.sneaking {
            self.vx *= 0.3;
            self.vz *= 0.3;
        }
        if climbing {
            self.vx *= 0.6;
            self.vz *= 0.6;
        }
        let on_ground = resolve_block_collisions(position, bounds, self, chunk);
        self.apply_friction(on_ground);
    }
}

/// Determine if the entity is in a fluid block such as water.
fn is_in_fluid(chunk: &Chunk, position: &Position, bounds: &CollisionBounds) -> bool {
    let mut found = false;
    for_each_block_in_bounds(chunk, position, bounds, |x, y, z| {
        if found {
            return;
        }
        if let Ok(block) = chunk.get_block(x, y, z) {
            if let Some(data) = block.to_block_data() {
                if data.name.contains("water") || data.name.contains("lava") {
                    found = true;
                }
            }
        }
        // Blocks outside the chunk are treated as empty space.
    });
    found
}

/// Check if the entity is touching a climbable block such as a ladder or vine.
fn is_touching_climbable(chunk: &Chunk, position: &Position, bounds: &CollisionBounds) -> bool {
    let mut found = false;
    for_each_block_in_bounds(chunk, position, bounds, |x, y, z| {
        if found {
            return;
        }
        if let Ok(block) = chunk.get_block(x, y, z) {
            if let Some(data) = block.to_block_data() {
                if data.name.contains("ladder") || data.name.contains("vine") {
                    found = true;
                }
            }
        }
        // Missing blocks simply don't provide climbable surfaces.
    });
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrumc_world::vanilla_chunk_format::BlockData;

    fn entity_bounds() -> CollisionBounds {
        CollisionBounds {
            x_offset_start: -0.3,
            x_offset_end: 0.3,
            y_offset_start: 0.0,
            y_offset_end: 1.8,
            z_offset_start: -0.3,
            z_offset_end: 0.3,
        }
    }

    #[test]
    fn falling_decreases_height() {
        let chunk = Chunk::new(0, 0, "overworld".to_string());
        let mut position = Position::new(0.0, 2.0, 0.0);
        let mut movement = Movement::default();
        movement.tick(&mut position, &entity_bounds(), &chunk);
        assert!(position.y < 2.0);
    }

    #[test]
    fn jumping_eventually_falls() {
        let chunk = Chunk::new(0, 0, "overworld".to_string());
        let mut position = Position::default();
        let mut movement = Movement::new(0.0, 0.42, 0.0);
        for _ in 0..10 {
            movement.tick(&mut position, &entity_bounds(), &chunk);
        }
        assert!(movement.vy < 0.0);
    }

    #[test]
    fn swimming_applies_buoyancy_and_drag() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let mut props = std::collections::BTreeMap::new();
        props.insert("level".to_string(), "0".to_string());
        let water = BlockData {
            name: "minecraft:water".to_string(),
            properties: Some(props),
        }
        .to_block_id();
        chunk.set_block(0, 0, 0, water).unwrap();
        let mut position = Position::new(0.5, 0.5, 0.5);
        let mut movement = Movement::new(0.2, -0.1, 0.0);
        movement.tick(&mut position, &entity_bounds(), &chunk);
        assert!(movement.vy > -0.1);
        assert!(movement.vx.abs() < 0.2);
    }

    #[test]
    fn collisions_stop_on_solid_blocks() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let stone = BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        }
        .to_block_id();
        chunk.set_block(0, 0, 0, stone).unwrap();
        let mut position = Position::new(0.5, 1.0, 0.5);
        let mut movement = Movement::new(0.0, -1.0, 0.0);
        movement.tick(&mut position, &entity_bounds(), &chunk);
        assert!(position.y >= 1.0);
        assert_eq!(movement.vy, 0.0);
    }
}
