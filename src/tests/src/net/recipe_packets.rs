use ferrumc_net::packets::incoming::{
    craft_recipe_request::CraftRecipeRequestPacket, displayed_recipe::DisplayedRecipePacket,
    recipe_book::RecipeBookPacket,
};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Cursor, Write};

#[test]
fn decode_craft_recipe_request() {
    let mut bytes = Vec::new();
    bytes.write_all(&[5u8]).unwrap();
    let recipe = b"minecraft:stone";
    VarInt::from(recipe.len() as i32).write(&mut bytes).unwrap();
    bytes.extend_from_slice(recipe);
    bytes.write_all(&[0u8]).unwrap();
    let mut cursor = Cursor::new(bytes);
    let pkt = CraftRecipeRequestPacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(pkt.window_id, 5);
    assert_eq!(pkt.recipe, "minecraft:stone");
    assert!(!pkt.make_all);
}

#[test]
fn decode_displayed_recipe() {
    let mut bytes = Vec::new();
    let recipe = b"minecraft:stone";
    VarInt::from(recipe.len() as i32).write(&mut bytes).unwrap();
    bytes.extend_from_slice(recipe);
    let mut cursor = Cursor::new(bytes);
    let pkt = DisplayedRecipePacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(pkt.recipe, "minecraft:stone");
}

#[test]
fn decode_recipe_book_packet() {
    let mut bytes = Vec::new();
    VarInt::from(1).write(&mut bytes).unwrap();
    bytes.write_all(&[1u8, 0u8]).unwrap();
    let mut cursor = Cursor::new(bytes);
    let pkt = RecipeBookPacket::decode(&mut cursor, &NetDecodeOpts::None).unwrap();
    assert_eq!(pkt.book_id.0, 1);
    assert!(pkt.open);
    assert!(!pkt.filtering);
}
