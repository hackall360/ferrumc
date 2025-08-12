use std::io::Write;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
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
