use crate::block_id::BlockId;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct RecipeJson {
    pattern: Vec<u32>,
    output: OutputJson,
}

#[derive(Debug, Clone, Deserialize)]
struct OutputJson {
    item: u32,
    count: u8,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub pattern: Vec<Option<BlockId>>,
    pub output: (BlockId, u8),
}

lazy_static! {
    pub static ref RECIPES: Vec<Recipe> = load_recipes();
}

fn load_recipes() -> Vec<Recipe> {
    let data = include_str!("../recipes/recipes.json");
    let parsed: Vec<RecipeJson> = serde_json::from_str(data).unwrap_or_default();
    parsed
        .into_iter()
        .map(|r| Recipe {
            pattern: r
                .pattern
                .into_iter()
                .map(|id| if id == 0 { None } else { Some(BlockId(id)) })
                .collect(),
            output: (BlockId(r.output.item), r.output.count),
        })
        .collect()
}

pub fn init() {
    lazy_static::initialize(&RECIPES);
}
