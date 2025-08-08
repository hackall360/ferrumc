use std::collections::HashSet;

use ferrumc_world::block_id::ID2BLOCK;

#[test]
fn no_1_21_blocks_present() {
    const V1_21_BLOCKS: &[&str] = &[
        "minecraft:chiseled_copper",
        "minecraft:chiseled_tuff",
        "minecraft:chiseled_tuff_bricks",
        "minecraft:copper_bulb",
        "minecraft:copper_door",
        "minecraft:copper_grate",
        "minecraft:copper_trapdoor",
        "minecraft:crafter",
        "minecraft:exposed_chiseled_copper",
        "minecraft:exposed_copper_bulb",
        "minecraft:exposed_copper_door",
        "minecraft:exposed_copper_grate",
        "minecraft:exposed_copper_trapdoor",
        "minecraft:heavy_core",
        "minecraft:oxidized_chiseled_copper",
        "minecraft:oxidized_copper_bulb",
        "minecraft:oxidized_copper_door",
        "minecraft:oxidized_copper_grate",
        "minecraft:oxidized_copper_trapdoor",
        "minecraft:polished_tuff",
        "minecraft:polished_tuff_slab",
        "minecraft:polished_tuff_stairs",
        "minecraft:polished_tuff_wall",
        "minecraft:short_grass",
        "minecraft:trial_spawner",
        "minecraft:tuff_brick_slab",
        "minecraft:tuff_brick_stairs",
        "minecraft:tuff_brick_wall",
        "minecraft:tuff_bricks",
        "minecraft:tuff_slab",
        "minecraft:tuff_stairs",
        "minecraft:tuff_wall",
        "minecraft:vault",
        "minecraft:waxed_chiseled_copper",
        "minecraft:waxed_copper_bulb",
        "minecraft:waxed_copper_door",
        "minecraft:waxed_copper_grate",
        "minecraft:waxed_copper_trapdoor",
        "minecraft:waxed_exposed_chiseled_copper",
        "minecraft:waxed_exposed_copper_bulb",
        "minecraft:waxed_exposed_copper_door",
        "minecraft:waxed_exposed_copper_grate",
        "minecraft:waxed_exposed_copper_trapdoor",
        "minecraft:waxed_oxidized_chiseled_copper",
        "minecraft:waxed_oxidized_copper_bulb",
        "minecraft:waxed_oxidized_copper_door",
        "minecraft:waxed_oxidized_copper_grate",
        "minecraft:waxed_oxidized_copper_trapdoor",
        "minecraft:waxed_weathered_chiseled_copper",
        "minecraft:waxed_weathered_copper_bulb",
        "minecraft:waxed_weathered_copper_door",
        "minecraft:waxed_weathered_copper_grate",
        "minecraft:waxed_weathered_copper_trapdoor",
        "minecraft:weathered_chiseled_copper",
        "minecraft:weathered_copper_bulb",
        "minecraft:weathered_copper_door",
        "minecraft:weathered_copper_grate",
        "minecraft:weathered_copper_trapdoor",
    ];
    let forbidden: HashSet<&str> = V1_21_BLOCKS.iter().copied().collect();
    for block in ID2BLOCK.iter() {
        assert!(
            !forbidden.contains(block.name.as_str()),
            "Found 1.21 block {} in block mappings",
            block.name
        );
    }
}
