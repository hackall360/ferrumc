use crate::player_data::{load_player_data, save_player_data, ItemStackData, PlayerStatsData, PositionData};
use crate::lmdb::LmdbBackend;
use tempfile::tempdir;

#[test]
#[ignore]
fn inventory_persists_across_sessions() {
    let dir = tempdir().unwrap();
    let db = LmdbBackend::initialize(Some(dir.path().to_path_buf())).unwrap();
    let uuid = 42u128;

    let mut pdata = load_player_data(&db, uuid).unwrap();
    pdata.inventory.hotbar.resize(9, None);
    assert!(pdata.inventory.hotbar.iter().all(|s| s.is_none()));

    pdata.inventory.hotbar[0] = Some(ItemStackData {
        item: 1,
        count: 5,
        max_stack_size: 64,
        nbt: None,
    });
    pdata.position = PositionData { x: 0.0, y: 64.0, z: 0.0 };
    pdata.stats = PlayerStatsData::default();
    save_player_data(&db, uuid, &pdata).unwrap();

    let loaded = load_player_data(&db, uuid).unwrap();
    assert_eq!(loaded.inventory.hotbar[0].as_ref().unwrap().count, 5);
}
