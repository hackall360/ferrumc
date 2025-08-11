use bevy_ecs::prelude::{Commands, Query, Res};
use ferrumc_state::GlobalStateResource;
use rand::Rng;
use tracing::error;

use crate::ai::{AIGoal, EntityKind, Mob, PendingSpawn};
use crate::collisions::bounds::CollisionBounds;
use crate::entities::spawn_rules::{self, SpawnRule};
use crate::identity::entity_id::EntityId;
use crate::movement::Movement;
use crate::transform::position::Position;
use crate::transform::rotation::Rotation;

/// System that advances the world by one tick.
pub fn tick_world(state: Res<GlobalStateResource>) {
    if let Err(e) = state.0.world.tick() {
        error!("World tick failed: {e}");
    }
}

/// Spawns entities based on biome-specific rules.
pub fn spawn_entities_from_biomes(
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
    existing: Query<&Mob>,
) {
    if !existing.is_empty() {
        return;
    }

    let biome = state.0.terrain_generator.biome_at(0, 0);
    let rules = spawn_rules::rules_for_biome(biome);
    if let Some(kind) = select_weighted(rules) {
        let id = EntityId::new(rand::random::<u128>());
        cmd.spawn((
            id,
            Mob { kind },
            Position::default(),
            Rotation::default(),
            Movement::default(),
            CollisionBounds {
                x_offset_start: -0.3,
                x_offset_end: 0.3,
                y_offset_start: 0.0,
                y_offset_end: 1.8,
                z_offset_start: -0.3,
                z_offset_end: 0.3,
            },
            AIGoal::Idle,
            PendingSpawn,
        ));
    }
}

fn select_weighted(rules: &[SpawnRule]) -> Option<EntityKind> {
    let total: u32 = rules.iter().map(|r| r.weight).sum();
    if total == 0 {
        return None;
    }
    let mut rng = rand::thread_rng();
    let mut roll = rng.gen_range(0..total);
    for rule in rules {
        if roll < rule.weight {
            return Some(rule.kind);
        }
        roll -= rule.weight;
    }
    None
}
