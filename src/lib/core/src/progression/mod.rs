use serde::Deserialize;
use std::collections::HashMap;

pub mod packets;
pub mod player;

pub use packets::AdvancementsPacket;
pub use player::PlayerProgression;

#[derive(Debug, Clone, Deserialize)]
pub struct Advancement(pub serde_json::Value);

impl Advancement {
    /// Returns all criteria names for this advancement.
    pub fn criteria(&self) -> Vec<String> {
        self.0
            .get("criteria")
            .and_then(|c| c.as_object())
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns recipe rewards associated with this advancement, if any.
    pub fn recipe_rewards(&self) -> Vec<String> {
        self.0
            .get("rewards")
            .and_then(|r| r.get("recipes"))
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe(pub serde_json::Value);

#[derive(Debug, Default, Clone)]
pub struct ProgressionRegistry {
    pub advancements: HashMap<String, Advancement>,
    pub recipes: HashMap<String, Recipe>,
}

impl ProgressionRegistry {
    pub fn from_json(advancements_json: &str, recipes_json: &str) -> serde_json::Result<Self> {
        let advancements: HashMap<String, Advancement> = serde_json::from_str(advancements_json)?;
        let recipes: HashMap<String, Recipe> = serde_json::from_str(recipes_json)?;
        Ok(Self {
            advancements,
            recipes,
        })
    }

    pub fn load() -> Self {
        const ADVANCEMENTS: &str = include_str!("../../../../../assets/data/advancements.json");
        const RECIPES: &str = include_str!("../../../../../assets/data/recipes.json");
        Self::from_json(ADVANCEMENTS, RECIPES).unwrap_or_default()
    }
}
