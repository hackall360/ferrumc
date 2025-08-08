mod biomes;
pub mod errors;

use crate::errors::WorldGenError;
use ferrumc_world::chunk_format::Chunk;
use noise::{Clamp, NoiseFn, OpenSimplex};

/// Trait for generating a biome
///
/// Should be implemented for each biome's generator
pub(crate) trait BiomeGenerator {
    fn _biome_id(&self) -> u8;
    fn _biome_name(&self) -> String;
    fn generate_chunk(
        &self,
        x: i32,
        z: i32,
        noise: &NoiseGenerator,
    ) -> Result<Chunk, WorldGenError>;
}

pub(crate) struct NoiseGenerator {
    pub(crate) layers: Vec<Clamp<f64, OpenSimplex, 2>>,
}

pub struct WorldGenerator {
    _seed: u64,
    noise_generator: NoiseGenerator,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        let mut layers = Vec::new();
        for i in 0..4 {
            let open_simplex = OpenSimplex::new((seed + i) as u32);
            let clamp = Clamp::new(open_simplex).set_bounds(-1.0, 1.0);
            layers.push(clamp);
        }
        Self { layers }
    }

    pub fn get_noise(&self, x: f64, z: f64) -> f64 {
        let mut noise = 0.0;
        for (c, layer) in self.layers.iter().enumerate() {
            let scale = 64.0_f64.powi(c as i32 + 1);
            noise += layer.get([x / scale, z / scale]);
        }
        noise / (self.layers.len() as f64 / 2.0)
    }
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            noise_generator: NoiseGenerator::new(seed),
        }
    }

    fn get_biome(&self, x: i32, z: i32) -> Box<dyn BiomeGenerator> {
        let id = self.biome_at(x, z);
        biomes::get_biome_by_id(id)
    }

    pub fn biome_at(&self, x: i32, z: i32) -> u8 {
        let noise = self.noise_generator.get_noise(x as f64, z as f64);
        let count = biomes::biome_count() as f64;
        let mut idx = ((noise + 1.0) / 2.0 * count).floor() as usize;
        if idx >= count as usize {
            idx = count as usize - 1;
        }
        idx as u8
    }

    pub fn generate_chunk_for_biome(
        &self,
        x: i32,
        z: i32,
        biome_id: u8,
    ) -> Result<Chunk, WorldGenError> {
        let biome = biomes::get_biome_by_id(biome_id);
        biome.generate_chunk(x, z, &self.noise_generator)
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let id = self.biome_at(x, z);
        self.generate_chunk_for_biome(x, z, id)
    }
}
