use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Advancement(pub serde_json::Value);

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe(pub serde_json::Value);

#[derive(Debug, Default, Clone)]
pub struct ProgressionRegistry {
    pub advancements: HashMap<String, Advancement>,
    pub recipes: HashMap<String, Recipe>,
}

impl ProgressionRegistry {
    pub fn from_json(
        advancements_json: &str,
        recipes_json: &str,
    ) -> serde_json::Result<Self> {
        let advancements: HashMap<String, Advancement> =
            serde_json::from_str(advancements_json)?;
        let recipes: HashMap<String, Recipe> = serde_json::from_str(recipes_json)?;
        Ok(Self {
            advancements,
            recipes,
        })
    }

    pub fn load() -> Self {
        const ADVANCEMENTS: &str = include_str!(
            "../../../../../assets/data/advancements.json"
        );
        const RECIPES: &str = include_str!(
            "../../../../../assets/data/recipes.json"
        );
        Self::from_json(ADVANCEMENTS, RECIPES).unwrap_or_default()
    }
}
