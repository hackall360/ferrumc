use crate::BiomeGenerator;

pub(crate) mod plains;
pub(crate) mod simple;
use simple::{SimpleBiome, Veg};

macro_rules! overworld_biome {
    ($mod_name:ident, $id:expr, $surface:expr, $filler:expr, $veg:expr, $scale:expr) => {
        pub(crate) mod $mod_name {
            use super::*;
            pub(crate) fn new() -> Box<dyn BiomeGenerator> {
                Box::new(SimpleBiome::new(
                    $id,
                    stringify!($mod_name),
                    "overworld",
                    "minecraft:stone",
                    $surface,
                    $filler,
                    $veg,
                    $scale,
                ))
            }
        }
    };
}

macro_rules! nether_biome {
    ($mod_name:ident, $id:expr, $surface:expr, $filler:expr) => {
        pub(crate) mod $mod_name {
            use super::*;
            pub(crate) fn new() -> Box<dyn BiomeGenerator> {
                Box::new(SimpleBiome::new(
                    $id,
                    stringify!($mod_name),
                    "the_nether",
                    "minecraft:netherrack",
                    $surface,
                    $filler,
                    Veg::None,
                    32.0,
                ))
            }
        }
    };
}

macro_rules! end_biome {
    ($mod_name:ident, $id:expr) => {
        pub(crate) mod $mod_name {
            use super::*;
            pub(crate) fn new() -> Box<dyn BiomeGenerator> {
                Box::new(SimpleBiome::new(
                    $id,
                    stringify!($mod_name),
                    "the_end",
                    "minecraft:end_stone",
                    "minecraft:end_stone",
                    "minecraft:end_stone",
                    Veg::None,
                    16.0,
                ))
            }
        }
    };
}

macro_rules! void_biome {
    ($mod_name:ident, $id:expr) => {
        pub(crate) mod $mod_name {
            use super::*;
            pub(crate) fn new() -> Box<dyn BiomeGenerator> {
                Box::new(SimpleBiome::new(
                    $id,
                    stringify!($mod_name),
                    "overworld",
                    "minecraft:air",
                    "minecraft:air",
                    "minecraft:air",
                    Veg::None,
                    0.0,
                ))
            }
        }
    };
}

// overworld biomes
overworld_biome!(badlands, 0, "minecraft:red_sand", "minecraft:red_sandstone", Veg::Cactus, 32.0);
overworld_biome!(bamboo_jungle, 1, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
nether_biome!(basalt_deltas, 2, "minecraft:basalt", "minecraft:basalt");
overworld_biome!(beach, 3, "minecraft:sand", "minecraft:sand", Veg::None, 32.0);
overworld_biome!(birch_forest, 4, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(cherry_grove, 5, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(cold_ocean, 6, "minecraft:gravel", "minecraft:gravel", Veg::None, 16.0);
nether_biome!(crimson_forest, 7, "minecraft:crimson_nylium", "minecraft:netherrack");
overworld_biome!(dark_forest, 8, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(deep_cold_ocean, 9, "minecraft:gravel", "minecraft:gravel", Veg::None, 16.0);
overworld_biome!(deep_dark, 10, "minecraft:stone", "minecraft:stone", Veg::None, 16.0);
overworld_biome!(deep_frozen_ocean, 11, "minecraft:gravel", "minecraft:gravel", Veg::None, 16.0);
overworld_biome!(deep_lukewarm_ocean, 12, "minecraft:sand", "minecraft:sand", Veg::None, 16.0);
overworld_biome!(deep_ocean, 13, "minecraft:gravel", "minecraft:gravel", Veg::None, 16.0);
overworld_biome!(desert, 14, "minecraft:sand", "minecraft:sand", Veg::Cactus, 32.0);
overworld_biome!(dripstone_caves, 15, "minecraft:stone", "minecraft:stone", Veg::None, 32.0);
end_biome!(end_barrens, 16);
end_biome!(end_highlands, 17);
end_biome!(end_midlands, 18);
overworld_biome!(eroded_badlands, 19, "minecraft:red_sand", "minecraft:red_sandstone", Veg::Cactus, 32.0);
overworld_biome!(flower_forest, 20, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
overworld_biome!(forest, 21, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(frozen_ocean, 22, "minecraft:ice", "minecraft:gravel", Veg::None, 16.0);
overworld_biome!(frozen_peaks, 23, "minecraft:snow_block", "minecraft:stone", Veg::None, 64.0);
overworld_biome!(frozen_river, 24, "minecraft:ice", "minecraft:stone", Veg::None, 16.0);
overworld_biome!(grove, 25, "minecraft:snow_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(ice_spikes, 26, "minecraft:snow_block", "minecraft:stone", Veg::None, 64.0);
overworld_biome!(jagged_peaks, 27, "minecraft:snow_block", "minecraft:stone", Veg::None, 64.0);
overworld_biome!(jungle, 28, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(lukewarm_ocean, 29, "minecraft:sand", "minecraft:sand", Veg::None, 16.0);
overworld_biome!(lush_caves, 30, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 32.0);
overworld_biome!(mangrove_swamp, 31, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 32.0);
overworld_biome!(meadow, 32, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
overworld_biome!(mushroom_fields, 33, "minecraft:mycelium", "minecraft:dirt", Veg::None, 32.0);
nether_biome!(nether_wastes, 34, "minecraft:netherrack", "minecraft:netherrack");
overworld_biome!(ocean, 35, "minecraft:sand", "minecraft:gravel", Veg::None, 16.0);
overworld_biome!(old_growth_birch_forest, 36, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(old_growth_pine_taiga, 37, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(old_growth_spruce_taiga, 38, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(river, 40, "minecraft:sand", "minecraft:sand", Veg::None, 16.0);
overworld_biome!(savanna, 41, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
overworld_biome!(savanna_plateau, 42, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
end_biome!(small_end_islands, 43);
overworld_biome!(snowy_beach, 44, "minecraft:snow_block", "minecraft:sand", Veg::None, 32.0);
overworld_biome!(snowy_plains, 45, "minecraft:snow_block", "minecraft:dirt", Veg::None, 64.0);
overworld_biome!(snowy_slopes, 46, "minecraft:snow_block", "minecraft:stone", Veg::None, 64.0);
overworld_biome!(snowy_taiga, 47, "minecraft:snow_block", "minecraft:dirt", Veg::OakTree, 64.0);
nether_biome!(soul_sand_valley, 48, "minecraft:soul_sand", "minecraft:soul_sand");
overworld_biome!(sparse_jungle, 49, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(stony_peaks, 50, "minecraft:stone", "minecraft:stone", Veg::None, 64.0);
overworld_biome!(stony_shore, 51, "minecraft:stone", "minecraft:gravel", Veg::None, 32.0);
overworld_biome!(sunflower_plains, 52, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
overworld_biome!(swamp, 53, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 32.0);
overworld_biome!(taiga, 54, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
end_biome!(the_end, 55);
void_biome!(the_void, 56);
overworld_biome!(warm_ocean, 57, "minecraft:sand", "minecraft:sand", Veg::None, 16.0);
nether_biome!(warped_forest, 58, "minecraft:warped_nylium", "minecraft:netherrack");
overworld_biome!(windswept_forest, 59, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(windswept_gravelly_hills, 60, "minecraft:gravel", "minecraft:gravel", Veg::None, 64.0);
overworld_biome!(windswept_hills, 61, "minecraft:grass_block", "minecraft:dirt", Veg::OakTree, 64.0);
overworld_biome!(windswept_savanna, 62, "minecraft:grass_block", "minecraft:dirt", Veg::Grass, 64.0);
overworld_biome!(wooded_badlands, 63, "minecraft:red_sand", "minecraft:red_sandstone", Veg::OakTree, 32.0);

pub(crate) const BIOME_CREATORS: [fn() -> Box<dyn BiomeGenerator>; 64] = [
    badlands::new,
    bamboo_jungle::new,
    basalt_deltas::new,
    beach::new,
    birch_forest::new,
    cherry_grove::new,
    cold_ocean::new,
    crimson_forest::new,
    dark_forest::new,
    deep_cold_ocean::new,
    deep_dark::new,
    deep_frozen_ocean::new,
    deep_lukewarm_ocean::new,
    deep_ocean::new,
    desert::new,
    dripstone_caves::new,
    end_barrens::new,
    end_highlands::new,
    end_midlands::new,
    eroded_badlands::new,
    flower_forest::new,
    forest::new,
    frozen_ocean::new,
    frozen_peaks::new,
    frozen_river::new,
    grove::new,
    ice_spikes::new,
    jagged_peaks::new,
    jungle::new,
    lukewarm_ocean::new,
    lush_caves::new,
    mangrove_swamp::new,
    meadow::new,
    mushroom_fields::new,
    nether_wastes::new,
    ocean::new,
    old_growth_birch_forest::new,
    old_growth_pine_taiga::new,
    old_growth_spruce_taiga::new,
    plains::new,
    river::new,
    savanna::new,
    savanna_plateau::new,
    small_end_islands::new,
    snowy_beach::new,
    snowy_plains::new,
    snowy_slopes::new,
    snowy_taiga::new,
    soul_sand_valley::new,
    sparse_jungle::new,
    stony_peaks::new,
    stony_shore::new,
    sunflower_plains::new,
    swamp::new,
    taiga::new,
    the_end::new,
    the_void::new,
    warm_ocean::new,
    warped_forest::new,
    windswept_forest::new,
    windswept_gravelly_hills::new,
    windswept_hills::new,
    windswept_savanna::new,
    wooded_badlands::new,
];

pub(crate) fn get_biome_by_id(id: u8) -> Box<dyn BiomeGenerator> {
    BIOME_CREATORS
        .get(id as usize)
        .map(|f| f())
        .unwrap_or_else(|| plains::new())
}

pub(crate) fn biome_count() -> usize {
    BIOME_CREATORS.len()
}
