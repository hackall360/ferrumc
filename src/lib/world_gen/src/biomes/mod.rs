use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;

pub(crate) mod plains;

macro_rules! biome {
    ($mod_name:ident, $id:expr) => {
        pub(crate) mod $mod_name {
            use super::*;
            pub(crate) struct Biome;
            impl BiomeGenerator for Biome {
                fn _biome_id(&self) -> u8 {
                    $id
                }
                fn _biome_name(&self) -> String {
                    stringify!($mod_name).to_string()
                }
                fn generate_chunk(
                    &self,
                    x: i32,
                    z: i32,
                    noise: &NoiseGenerator,
                ) -> Result<Chunk, WorldGenError> {
                    super::plains::PlainsBiome.generate_chunk(x, z, noise)
                }
            }
            pub(crate) fn new() -> Box<dyn BiomeGenerator> {
                Box::new(Biome)
            }
        }
    };
}

biome!(badlands, 0);
biome!(bamboo_jungle, 1);
biome!(basalt_deltas, 2);
biome!(beach, 3);
biome!(birch_forest, 4);
biome!(cherry_grove, 5);
biome!(cold_ocean, 6);
biome!(crimson_forest, 7);
biome!(dark_forest, 8);
biome!(deep_cold_ocean, 9);
biome!(deep_dark, 10);
biome!(deep_frozen_ocean, 11);
biome!(deep_lukewarm_ocean, 12);
biome!(deep_ocean, 13);
biome!(desert, 14);
biome!(dripstone_caves, 15);
biome!(end_barrens, 16);
biome!(end_highlands, 17);
biome!(end_midlands, 18);
biome!(eroded_badlands, 19);
biome!(flower_forest, 20);
biome!(forest, 21);
biome!(frozen_ocean, 22);
biome!(frozen_peaks, 23);
biome!(frozen_river, 24);
biome!(grove, 25);
biome!(ice_spikes, 26);
biome!(jagged_peaks, 27);
biome!(jungle, 28);
biome!(lukewarm_ocean, 29);
biome!(lush_caves, 30);
biome!(mangrove_swamp, 31);
biome!(meadow, 32);
biome!(mushroom_fields, 33);
biome!(nether_wastes, 34);
biome!(ocean, 35);
biome!(old_growth_birch_forest, 36);
biome!(old_growth_pine_taiga, 37);
biome!(old_growth_spruce_taiga, 38);
biome!(river, 40);
biome!(savanna, 41);
biome!(savanna_plateau, 42);
biome!(small_end_islands, 43);
biome!(snowy_beach, 44);
biome!(snowy_plains, 45);
biome!(snowy_slopes, 46);
biome!(snowy_taiga, 47);
biome!(soul_sand_valley, 48);
biome!(sparse_jungle, 49);
biome!(stony_peaks, 50);
biome!(stony_shore, 51);
biome!(sunflower_plains, 52);
biome!(swamp, 53);
biome!(taiga, 54);
biome!(the_end, 55);
biome!(the_void, 56);
biome!(warm_ocean, 57);
biome!(warped_forest, 58);
biome!(windswept_forest, 59);
biome!(windswept_gravelly_hills, 60);
biome!(windswept_hills, 61);
biome!(windswept_savanna, 62);
biome!(wooded_badlands, 63);

pub(crate) fn plains_new() -> Box<dyn BiomeGenerator> {
    Box::new(plains::PlainsBiome)
}

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
    plains_new,
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
        .unwrap_or_else(|| plains_new())
}

pub(crate) fn biome_count() -> usize {
    BIOME_CREATORS.len()
}
