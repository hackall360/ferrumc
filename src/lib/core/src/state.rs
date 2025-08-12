use bevy_ecs::prelude::{Commands, Query, Res};
use ferrumc_state::GlobalStateResource;
use ferrumc_storage::errors::StorageError;
use ferrumc_storage::lmdb::LmdbBackend;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::ai::{AIGoal, EntityKind, Mob, PendingSpawn};
use crate::collisions::bounds::CollisionBounds;
use crate::entities::spawn_rules::{self, SpawnRule};
use crate::identity::entity_id::EntityId;
use crate::movement::Movement;
use crate::transform::position::Position;
use crate::transform::rotation::Rotation;
use crate::furnace::{furnace_tick, Furnace};
use crate::brewing::{brewing_tick, BrewingStand};
use crate::inventory::Inventory;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerStats {
    pub deaths: u32,
    pub mobs_killed: u32,
}

pub fn load_player_stats(
    db: &LmdbBackend,
    uuid: u128,
) -> Result<PlayerStats, StorageError> {
    if let Some(bytes) = db.get("player_stats".to_string(), uuid)? {
        serde_json::from_slice(&bytes)
            .map_err(|e| StorageError::ReadError(e.to_string()))
    } else {
        Ok(PlayerStats::default())
    }
}

pub fn save_player_stats(
    db: &LmdbBackend,
    uuid: u128,
    stats: &PlayerStats,
) -> Result<(), StorageError> {
    let bytes =
        serde_json::to_vec(stats).map_err(|e| StorageError::WriteError(e.to_string()))?;
    db.upsert("player_stats".to_string(), uuid, bytes)?;
    Ok(())
}

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

pub fn process_furnaces(query: Query<(&mut Furnace, &mut Inventory)>) {
    furnace_tick(query);
}

pub fn process_brewing_stands(query: Query<(&mut BrewingStand, &mut Inventory)>) {
    brewing_tick(query);
}
