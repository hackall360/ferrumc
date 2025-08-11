use std::sync::Arc;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumc_config::server_config::{set_global_config, DatabaseConfig, ServerConfig};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::{
    block_entities::chest::{ChestBlockEntity, ChestItem},
    chunk_format::Chunk,
    World,
};

fn init_config() {
    static INIT: Once = Once::new();
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
}

#[test]
fn chest_persist_reload() {
    init_config();
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("ferrumc_world_chest_{}", nanos));
    std::fs::create_dir_all(&path).unwrap();

    let world = World::new(&path);
    let mut chunk = Chunk::new(0, 0, "overworld".to_string());

    let chest = ChestBlockEntity {
        items: vec![ChestItem {
            slot: 0,
            id: "minecraft:stone".to_string(),
            count: 1,
        }],
    };
    chunk.set_block_entity_data(0, 64, 0, VarInt::from(0), &chest);
    world.save_chunk(Arc::new(chunk)).unwrap();
    drop(world);

    let world = World::new(&path);
    let chunk = world.load_chunk_owned(0, 0, "overworld").unwrap();
    let loaded: ChestBlockEntity = chunk.get_block_entity_data(0, 64, 0).expect("block entity");
    assert_eq!(loaded.items.len(), 1);
    assert_eq!(loaded.items[0].id, "minecraft:stone");
    assert_eq!(loaded.items[0].count, 1);
}
