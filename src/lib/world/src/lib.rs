pub mod biome_id;
pub mod block_entities;
pub mod block_id;
pub mod chunk_format;
mod db_functions;
pub mod edit_batch;
pub mod edits;
pub mod errors;
mod importing;
pub mod redstone;
pub mod tick;
pub mod vanilla_chunk_format;
pub mod recipes;

use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use deepsize::DeepSizeOf;
use ferrumc_config::server_config::get_global_config;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_storage::lmdb::LmdbBackend;
use moka::{notification::RemovalCause, sync::Cache};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{error, trace, warn};

pub const OVERWORLD_ID: i32 = 0;
pub const NETHER_ID: i32 = -1;
pub const END_ID: i32 = 1;

pub fn dimension_id(name: &str) -> i32 {
    match name {
        "overworld" | "minecraft:overworld" => OVERWORLD_ID,
        "nether" | "minecraft:the_nether" => NETHER_ID,
        "end" | "minecraft:the_end" => END_ID,
        _ => OVERWORLD_ID,
    }
}

pub fn dimension_name(id: i32) -> &'static str {
    match id {
        NETHER_ID => "nether",
        END_ID => "end",
        _ => "overworld",
    }
}

pub fn dimension_registry_name(id: i32) -> &'static str {
    match id {
        NETHER_ID => "minecraft:the_nether",
        END_ID => "minecraft:the_end",
        _ => "minecraft:overworld",
    }
}

#[derive(Clone)]
pub struct World {
    storage_backend: LmdbBackend,
    cache: Cache<(i32, i32, String), Arc<Chunk>>,
    pub(crate) tick_manager: Arc<Mutex<tick::TickManager>>,
    pub(crate) redstone_cache: Arc<Mutex<redstone::PowerLevelCache>>,
}

fn check_config_validity() -> Result<(), WorldError> {
    // We don't actually check if the import path is valid here since that would brick a server
    // if the world is imported then deleted after the server starts. Those checks are handled in
    // the importing logic.

    let config = get_global_config();
    let db_path = get_root_path().join(&config.database.db_path);

    if config.database.map_size == 0 {
        error!("Map size is set to 0. Please set the map size in the configuration file.");
        return Err(WorldError::InvalidMapSize(config.database.map_size));
    }
    if !Path::new(&db_path).exists() {
        warn!("World path does not exist. Attempting to create it.");
        if create_dir_all(&db_path).is_err() {
            error!("Could not create world path: {}", db_path.display());
            return Err(WorldError::InvalidWorldPath(
                db_path.to_string_lossy().to_string(),
            ));
        }
    }
    if Path::new(&db_path).is_file() {
        error!("World path is a file. Please set the world path to a directory.");
        return Err(WorldError::InvalidWorldPath(
            db_path.to_string_lossy().to_string(),
        ));
    }
    if let Err(e) = Path::new(&db_path).read_dir() {
        error!("Could not read world path: {}", e);
        return Err(WorldError::InvalidWorldPath(
            db_path.to_string_lossy().to_string(),
        ));
    }

    for dim in ["DIM-1", "DIM1"] {
        let dim_path = db_path.join(dim);
        if !dim_path.exists() {
            if create_dir_all(&dim_path).is_err() {
                error!("Could not create world path: {}", dim_path.display());
                return Err(WorldError::InvalidWorldPath(
                    dim_path.to_string_lossy().to_string(),
                ));
            }
        }
    }

    // Check if doing map_size * 1024^3 would overflow usize. You probably don't need a database
    // that's 18 exabytes anyway.
    if config.database.map_size as usize > ((usize::MAX / 1024) / 1024) / 1024 {
        error!(
            "Map size is too large, this would exceed the usize limit. You probably don't need a \
        database this big anyway. Are you sure you have set the map size in GB, not bytes?"
        );
        return Err(WorldError::InvalidMapSize(config.database.map_size));
    }
    Ok(())
}

impl World {
    /// Creates a new world instance.
    ///
    /// You'd probably want to call this at the start of your program. And then use the returned
    /// in a state struct or something.
    pub fn new(backend_path: impl Into<PathBuf>) -> Self {
        recipes::init();
        if let Err(e) = check_config_validity() {
            error!("Fatal error in database config: {}", e);
            exit(1);
        }
        let mut backend_path = backend_path.into();
        // Clones are kinda ok here since this is only run once at startup.
        if backend_path.is_relative() {
            backend_path = get_root_path().join(backend_path);
        }
        let storage_backend =
            LmdbBackend::initialize(Some(backend_path)).expect("Failed to initialize database");

        if get_global_config().database.cache_ttl != 0
            && get_global_config().database.cache_capacity == 0
        {
            error!("Cache TTL and capacity must both be set to 0 or both be set to a value greater than 0.");
            exit(1);
        }

        let tick_manager = Arc::new(Mutex::new(tick::TickManager::default()));
        let redstone_cache = Arc::new(Mutex::new(redstone::PowerLevelCache::default()));
        let tm_clone = Arc::clone(&tick_manager);
        let eviction_listener =
            move |key: Arc<(i32, i32, String)>, _: Arc<Chunk>, cause: RemovalCause| {
                trace!("Evicting key: {:?}, cause: {:?}", key, cause);
                let (cx, cz, dim) = &*key;
                tm_clone.lock().unwrap().cleanup_chunk(*cx, *cz, dim);
            };

        let cache = Cache::builder()
            .eviction_listener(eviction_listener)
            .weigher(|_k, v: &Arc<Chunk>| v.deep_size_of() as u32)
            .time_to_live(Duration::from_secs(get_global_config().database.cache_ttl))
            .max_capacity(get_global_config().database.cache_capacity * 1024)
            .build();

        World {
            storage_backend,
            cache,
            tick_manager,
            redstone_cache,
        }
    }

    /// Ticks the world, processing scheduled and random block updates.
    pub fn tick(&self) -> Result<(), WorldError> {
        self.tick_manager.lock().unwrap().tick_world(self)
    }

    /// Schedule a future tick for the block at the given position.
    pub fn schedule_tick(&self, x: i32, y: i32, z: i32, dimension: &str, delay: u32) {
        tick::schedule_block_tick(self, x, y, z, dimension, delay);
    }

    /// Register a block position for random ticking.
    pub fn schedule_random_tick(&self, x: i32, y: i32, z: i32, dimension: &str) {
        tick::schedule_random_tick(self, x, y, z, dimension);
    }

    /// Get cached redstone power level at position.
    pub fn get_power_level(&self, x: i32, y: i32, z: i32, dimension: &str) -> u8 {
        let cache = self.redstone_cache.lock().unwrap();
        *cache
            .levels
            .get(&(x, y, z, dimension.to_string()))
            .unwrap_or(&0)
    }

    /// Remove all pending ticks associated with a dimension.
    pub fn unload_dimension(&self, dimension: &str) {
        self.tick_manager
            .lock()
            .unwrap()
            .cleanup_dimension(dimension);
    }

    pub fn backend(&self) -> &LmdbBackend {
        &self.storage_backend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn dump_chunk() {
        let world = World::new(
            std::env::current_dir()
                .unwrap()
                .join("../../../target/debug/world"),
        );
        let chunk = world.load_chunk(1, 1, "overworld").expect(
            "Failed to load chunk. If it's a bitcode error, chances are the chunk format \
             has changed since last generating a world so you'll need to regenerate",
        );
        let encoded = bitcode::encode(&chunk);
        std::fs::write("../../../.etc/raw_chunk.dat", encoded).unwrap();
    }
}
