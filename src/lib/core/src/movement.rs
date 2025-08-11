use bevy_ecs::prelude::Component;
use ferrumc_world::chunk_format::Chunk;
use typename::TypeName;

use crate::{collisions::block::block_bounds, transform::position::Position};

#[derive(TypeName, Component, Debug, Clone, Copy, Default)]
pub struct Movement {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

impl Movement {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self { vx, vy, vz }
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
    pub fn tick(&mut self, position: &mut Position, chunk: &Chunk) {
        let in_fluid = is_in_fluid(chunk, position);
        let on_ground = is_on_ground(chunk, position);
        self.apply_gravity(in_fluid);
        self.apply_water_flow(in_fluid);
        self.apply_drag(in_fluid);
        self.apply_friction(on_ground);
        position.x += self.vx;
        position.y += self.vy;
        position.z += self.vz;
    }
}

/// Determine if the entity is in a fluid block such as water.
fn is_in_fluid(chunk: &Chunk, position: &Position) -> bool {
    let bx = position.x.floor() as i32;
    let by = position.y.floor() as i32;
    let bz = position.z.floor() as i32;
    if let Ok(block) = chunk.get_block(bx, by, bz) {
        if let Some(data) = block.to_block_data() {
            return data.name.contains("water") || data.name.contains("lava");
        }
    }
    false
}

/// Basic ground check: returns true if the block directly below has collision bounds.
fn is_on_ground(chunk: &Chunk, position: &Position) -> bool {
    let bx = position.x.floor() as i32;
    let by = position.y.floor() as i32 - 1;
    let bz = position.z.floor() as i32;
    block_bounds(chunk, bx, by, bz).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrumc_world::vanilla_chunk_format::BlockData;

    #[test]
    fn falling_decreases_height() {
        let chunk = Chunk::new(0, 0, "overworld".to_string());
        let mut position = Position::new(0.0, 2.0, 0.0);
        let mut movement = Movement::default();
        movement.tick(&mut position, &chunk);
        assert!(position.y < 2.0);
    }

    #[test]
    fn jumping_eventually_falls() {
        let chunk = Chunk::new(0, 0, "overworld".to_string());
        let mut position = Position::default();
        let mut movement = Movement::new(0.0, 0.42, 0.0);
        for _ in 0..10 {
            movement.tick(&mut position, &chunk);
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
        movement.tick(&mut position, &chunk);
        assert!(movement.vy > -0.1);
        assert!(movement.vx.abs() < 0.2);
    }
}
