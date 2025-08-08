use crate::biomes;
use crate::errors::WorldGenError;
use crate::{BiomeGenerator, DimensionGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;

pub struct OverworldGenerator;

impl OverworldGenerator {
    fn get_biome(&self, _x: i32, _z: i32) -> Box<dyn BiomeGenerator> {
        Box::new(biomes::plains::PlainsBiome)
    }
}

impl DimensionGenerator for OverworldGenerator {
    fn generate_chunk(
        &self,
        x: i32,
        z: i32,
        noise: &NoiseGenerator,
    ) -> Result<Chunk, WorldGenError> {
        let biome = self.get_biome(x, z);
        biome.generate_chunk(x, z, noise)
    }
}
