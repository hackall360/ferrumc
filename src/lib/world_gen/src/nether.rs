use crate::biomes::simple::{SimpleBiome, Veg};
use crate::errors::WorldGenError;
use crate::noise_settings::NETHER_NOISE_SETTINGS;
use crate::structures::{temple::Temple, StructurePlacer};
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;

/// Basic nether terrain generator.
pub struct NetherGenerator {
    noise: NoiseGenerator,
    biome: SimpleBiome,
    structures: Vec<Box<dyn StructurePlacer + Send + Sync>>,
    seed: u64,
}

impl NetherGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            noise: NoiseGenerator::new(seed, NETHER_NOISE_SETTINGS),
            biome: SimpleBiome::new(
                0,
                "minecraft:nether_wastes",
                "the_nether",
                "minecraft:netherrack",
                "minecraft:netherrack",
                "minecraft:netherrack",
                Veg::None,
                32.0,
            ),
            structures: vec![Box::new(Temple)],
            seed,
        }
    }

    fn apply_structures(&self, chunk: &mut Chunk) {
        for s in &self.structures {
            s.place(chunk, self.seed);
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = self.biome.generate_chunk(x, z, &self.noise)?;
        self.apply_structures(&mut chunk);
        Ok(chunk)
    }
}
