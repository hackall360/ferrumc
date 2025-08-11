use std::collections::BTreeMap;

use ferrumc_world::World;
use ferrumc_world::vanilla_chunk_format::BlockData;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::tick::{TickManager, ScheduledTick, BlockPos};
use ferrumc_config::server_config::{set_global_config, ServerConfig, DatabaseConfig};
use std::sync::{Mutex, Once, Arc};

fn setup_world() -> World {
    static INIT: Once = Once::new();
    static LOCK: Mutex<()> = Mutex::new(());
    let _guard = LOCK.lock().unwrap();
    INIT.call_once(|| {
        let mut base = std::env::temp_dir();
        base.push("ferrumc_world_tests");
        std::fs::create_dir_all(&base).unwrap();
        let cfg = ServerConfig {
            database: DatabaseConfig {
                db_path: base.to_string_lossy().to_string(),
                map_size: 1,
                cache_ttl: 0,
                cache_capacity: 0,
                verify_chunk_data: false,
            },
            ..Default::default()
        };
        set_global_config(cfg);
    });
    let mut path = std::env::temp_dir();
    path.push(format!("ferrumc_world_tests_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    std::fs::create_dir_all(&path).unwrap();
    let world = World::new(&path);
    // Ensure the chunk table exists by inserting a dummy chunk
    let chunk = Chunk::new(0, 0, "overworld".to_string());
    world.save_chunk(Arc::new(chunk)).unwrap();
    world
}

#[test]
#[ignore]
fn water_spreads_downward() {
    let world = setup_world();
    let mut props = BTreeMap::new();
    props.insert("level".to_string(), "0".to_string());
    let water = BlockData { name: "minecraft:water".to_string(), properties: Some(props) };
    world.set_block_and_fetch(0, 1, 0, "overworld", water).unwrap();
    world.schedule_tick(0, 1, 0, "overworld", 0);
    world.tick().unwrap();
    let below = world.get_block_and_fetch(0, 0, 0, "overworld").unwrap();
    let data = below.to_block_data().unwrap();
    assert_eq!(data.name, "minecraft:water");
}

#[test]
#[ignore]
fn crops_grow_over_time() {
    let world = setup_world();
    let mut props = BTreeMap::new();
    props.insert("age".to_string(), "0".to_string());
    let wheat = BlockData { name: "minecraft:wheat".to_string(), properties: Some(props) };
    world.set_block_and_fetch(0, 1, 0, "overworld", wheat).unwrap();
    world.schedule_random_tick(0, 1, 0, "overworld");
    for _ in 0..3 {
        world.tick().unwrap();
    }
    let plant = world.get_block_and_fetch(0, 1, 0, "overworld").unwrap();
    let data = plant.to_block_data().unwrap();
    let age = data
        .properties
        .unwrap()
        .get("age")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap();
    assert!(age > 0);
}

#[test]
fn cleanup_chunk_removes_entries() {
    let mut tm = TickManager::default();
    let pos = BlockPos { x: 0, y: 0, z: 0, dimension: "overworld".to_string() };
    tm.schedule(ScheduledTick { pos: pos.clone(), block: BlockId::default(), delay: 0 });
    tm.schedule_random(pos.clone());
    let key = (0, 0, "overworld".to_string());
    assert!(tm.scheduled.contains_key(&key));
    assert!(tm.random.contains_key(&key));
    tm.cleanup_chunk(0, 0, "overworld");
    assert!(!tm.scheduled.contains_key(&key));
    assert!(!tm.random.contains_key(&key));
}

#[test]
fn cleanup_dimension_removes_all_matches() {
    let mut tm = TickManager::default();
    let over = BlockPos { x: 0, y: 0, z: 0, dimension: "overworld".to_string() };
    let nether = BlockPos { x: 0, y: 0, z: 0, dimension: "nether".to_string() };
    tm.schedule(ScheduledTick { pos: over.clone(), block: BlockId::default(), delay: 0 });
    tm.schedule_random(over.clone());
    tm.schedule(ScheduledTick { pos: nether.clone(), block: BlockId::default(), delay: 0 });
    tm.schedule_random(nether.clone());
    tm.cleanup_dimension("overworld");
    let key_over = (0, 0, "overworld".to_string());
    let key_nether = (0, 0, "nether".to_string());
    assert!(!tm.scheduled.contains_key(&key_over));
    assert!(!tm.random.contains_key(&key_over));
    assert!(tm.scheduled.contains_key(&key_nether));
    assert!(tm.random.contains_key(&key_nether));
}
