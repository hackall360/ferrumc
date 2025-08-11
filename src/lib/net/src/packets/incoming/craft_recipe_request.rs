use ferrumc_macros::{NetDecode, packet};

/// Sent when the player clicks a recipe in the recipe book to request the
/// server craft it. Maps to the serverbound `Craft Recipe Request` packet
/// described in the Minecraft protocol (v763, Play state).
#[derive(NetDecode, Debug)]
#[packet(packet_id = "craft_recipe_request", state = "play")]
pub struct CraftRecipeRequestPacket {
    /// ID of the window the recipe is crafted in.
    pub window_id: i8,
    /// Namespaced identifier of the recipe selected by the client.
    pub recipe: String,
    /// Whether to craft as many results as possible rather than a single item.
    pub make_all: bool,
}
