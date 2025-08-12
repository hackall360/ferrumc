use ferrumc_macros::{NBTDeserialize, NBTSerialize};
use ferrumc_nbt::{FromNbt, NBTSerializable, NBTSerializeOptions, NbtTape};
use serde::{Deserialize, Serialize};

use crate::errors::StorageError;
use crate::lmdb::LmdbBackend;

#[derive(Debug, Clone, Default, Serialize, Deserialize, NBTSerialize, NBTDeserialize)]
pub struct ItemStackData {
    pub item: u32,
    pub count: u8,
    pub max_stack_size: u8,
    pub nbt: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, NBTSerialize, NBTDeserialize)]
pub struct InventoryData {
    pub hotbar: Vec<Option<ItemStackData>>,
    pub main: Vec<Option<ItemStackData>>,
    pub equipment: Vec<Option<ItemStackData>>,
    pub offhand: Option<ItemStackData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, NBTSerialize, NBTDeserialize)]
pub struct PositionData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, NBTSerialize, NBTDeserialize)]
pub struct PlayerStatsData {
    pub deaths: u32,
    pub mobs_killed: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, NBTSerialize, NBTDeserialize)]
pub struct PlayerData {
    pub inventory: InventoryData,
    pub position: PositionData,
    pub stats: PlayerStatsData,
    pub advancements: Vec<String>,
}

pub fn save_player_data(
    db: &LmdbBackend,
    uuid: u128,
    data: &PlayerData,
) -> Result<(), StorageError> {
    let mut buf = Vec::new();
    NBTSerializable::serialize(data, &mut buf, &NBTSerializeOptions::WithHeader(""));
    if !db.table_exists("player_data".to_string())? {
        db.create_table("player_data".to_string())?;
    }
    db.upsert("player_data".to_string(), uuid, buf)?;
    Ok(())
}

pub fn load_player_data(db: &LmdbBackend, uuid: u128) -> Result<PlayerData, StorageError> {
    match db.get("player_data".to_string(), uuid) {
        Ok(Some(bytes)) => {
            let mut tape = NbtTape::new(&bytes);
            tape.parse();
            let (_, root) = tape
                .root
                .as_ref()
                .ok_or_else(|| StorageError::ReadError("No root tag".into()))?;
            FromNbt::from_nbt(&tape, root).map_err(|e| StorageError::ReadError(e.to_string()))
        }
        Ok(None) => Ok(PlayerData::default()),
        Err(StorageError::TableError(_)) => Ok(PlayerData::default()),
        Err(e) => Err(e),
    }
}

