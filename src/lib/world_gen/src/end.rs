use crate::biomes::simple::{SimpleBiome, Veg};
use crate::errors::WorldGenError;
use crate::noise_settings::END_NOISE_SETTINGS;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;

/// Basic end terrain generator.
pub struct EndGenerator {
    noise: NoiseGenerator,
    biome: SimpleBiome,
}

impl EndGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            noise: NoiseGenerator::new(seed, END_NOISE_SETTINGS),
            biome: SimpleBiome::new(
                0,
                "minecraft:the_end",
                "end",
                "minecraft:end_stone",
                "minecraft:end_stone",
                "minecraft:end_stone",
                Veg::None,
                32.0,
            ),
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        self.biome.generate_chunk(x, z, &self.noise)
    }
}
