use ferrumc_macros::{NetDecode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Notifies the server that the player opened or changed the filtering state
/// of a recipe book tab. Corresponds to the serverbound `Recipe Book` packet
/// in the Minecraft protocol (v763, Play state).
#[derive(NetDecode, Debug)]
#[packet(packet_id = "recipe_book", state = "play")]
pub struct RecipeBookPacket {
    /// Which recipe book tab is affected (0: crafting, 1: furnace, 2: blast furnace, 3: smoker).
    pub book_id: VarInt,
    /// Whether the tab should be opened in the client's GUI.
    pub open: bool,
    /// Whether the book should filter out unlocked recipes.
    pub filtering: bool,
}
