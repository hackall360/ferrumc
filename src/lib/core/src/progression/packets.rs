use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode, Clone)]
pub struct AdvancementProgressItem {
    pub criterion_id: String,
    pub achieved: bool,
}

#[derive(NetEncode, Clone)]
pub struct AdvancementProgressData {
    pub advancement_id: String,
    pub progress: Vec<AdvancementProgressItem>,
}

#[derive(NetEncode, Clone)]
#[packet(packet_id = "advancements", state = "play")]
pub struct AdvancementsPacket {
    pub reset: bool,
    pub progress: Vec<AdvancementProgressData>,
}

impl AdvancementsPacket {
    pub fn grant_single(id: impl Into<String>) -> Self {
        Self {
            reset: false,
            progress: vec![AdvancementProgressData {
                advancement_id: id.into(),
                progress: Vec::new(),
            }],
        }
    }
}

// Duplicates ferrumc-net's CraftedRecipePacket to avoid a cyclic dependency.
#[derive(NetEncode, Clone)]
#[packet(packet_id = "crafted_recipe", state = "play")]
pub struct CraftedRecipePacket {
    pub recipe_id: String,
}

impl CraftedRecipePacket {
    pub fn new(recipe_id: impl Into<String>) -> Self {
        Self {
            recipe_id: recipe_id.into(),
        }
    }
}
