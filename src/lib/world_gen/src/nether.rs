use crate::biomes::simple::{SimpleBiome, Veg};
use crate::errors::WorldGenError;
use crate::noise_settings::NETHER_NOISE_SETTINGS;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;

/// Basic nether terrain generator.
pub struct NetherGenerator {
    noise: NoiseGenerator,
    biome: SimpleBiome,
}

impl NetherGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            noise: NoiseGenerator::new(seed, NETHER_NOISE_SETTINGS),
            biome: SimpleBiome::new(
                0,
                "minecraft:nether_wastes",
                "nether",
                "minecraft:netherrack",
                "minecraft:netherrack",
                "minecraft:netherrack",
                Veg::None,
                32.0,
            ),
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        self.biome.generate_chunk(x, z, &self.noise)
    }
}
