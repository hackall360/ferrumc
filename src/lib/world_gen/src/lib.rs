mod biomes;
mod dimensions;
pub mod errors;

use crate::errors::WorldGenError;
use dashmap::DashMap;
use dimensions::overworld::OverworldGenerator;
use ferrumc_world::chunk_format::Chunk;
use noise::{Clamp, NoiseFn, OpenSimplex};
use std::sync::Arc;

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

pub trait DimensionGenerator: Send + Sync {
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
    generators: DashMap<String, Arc<dyn DimensionGenerator>>,
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
        let generator = WorldGenerator {
            _seed: seed,
            noise_generator: NoiseGenerator::new(seed),
            generators: DashMap::new(),
        };
        generator
            .generators
            .insert("overworld".to_string(), Arc::new(OverworldGenerator));
        generator
    }

    pub fn generate_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Chunk, WorldGenError> {
        if let Some(generator) = self.generators.get(dimension) {
            generator.generate_chunk(x, z, &self.noise_generator)
        } else {
            Err(WorldGenError::InvalidDimension)
        }
    }
}
