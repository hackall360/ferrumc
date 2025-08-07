use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::error;

#[derive(Deserialize)]
struct BiomeEntry {
    id: i32,
    name: String,
}

lazy_static! {
    pub static ref BIOME_NAME_TO_ID: HashMap<String, i32> = {
        let json_str = include_str!("../../../../.etc/biomes.json");
        let entries: Vec<BiomeEntry> =
            serde_json::from_str(json_str).expect("Failed to parse biomes.json");
        entries
            .into_iter()
            .map(|e| (format!("minecraft:{}", e.name), e.id))
            .collect()
    };
}

/// Retrieve the biome id for a given biome name.
pub fn get_biome_id(name: &str) -> Option<i32> {
    if let Some(id) = BIOME_NAME_TO_ID.get(name) {
        Some(*id)
    } else {
        error!("Could not find biome id for palette entry: {}", name);
        None
    }
}
