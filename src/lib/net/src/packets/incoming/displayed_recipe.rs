use ferrumc_macros::{NetDecode, packet};

/// Indicates which recipe the client is currently displaying in the recipe
/// book interface. Maps to the serverbound `Displayed Recipe` packet in the
/// Minecraft protocol (v763, Play state).
#[derive(NetDecode, Debug)]
#[packet(packet_id = "displayed_recipe", state = "play")]
pub struct DisplayedRecipePacket {
    /// Namespaced identifier of the recipe being shown.
    pub recipe: String,
}
