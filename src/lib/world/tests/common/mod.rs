use ferrumc_world::World;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_config::server_config::{set_global_config, ServerConfig, DatabaseConfig};
use std::sync::{Mutex, Once, Arc};

pub fn setup_world() -> World {
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
    let chunk = Chunk::new(0, 0, "overworld".to_string());
    world.save_chunk(Arc::new(chunk)).unwrap();
    world
}
