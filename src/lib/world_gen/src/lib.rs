mod biomes;
pub mod errors;
mod noise_settings;
mod structures;

use crate::errors::WorldGenError;
use ferrumc_world::chunk_format::Chunk;
use noise::{Clamp, NoiseFn, OpenSimplex};
use noise_settings::{NoiseSettings, OVERWORLD_NOISE_SETTINGS};
use structures::{temple::Temple, village::Village, StructurePlacer};

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
    pub(crate) settings: NoiseSettings,
}

pub struct WorldGenerator {
    _seed: u64,
    noise_generator: NoiseGenerator,
    structures: Vec<Box<dyn StructurePlacer + Send + Sync>>,
}

impl NoiseGenerator {
    pub fn new(seed: u64, settings: NoiseSettings) -> Self {
        let mut layers = Vec::new();
        for i in 0..4 {
            let open_simplex = OpenSimplex::new((seed + i) as u32);
            let clamp = Clamp::new(open_simplex).set_bounds(-1.0, 1.0);
            layers.push(clamp);
        }
        Self { layers, settings }
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
        let noise_generator = NoiseGenerator::new(seed, OVERWORLD_NOISE_SETTINGS);
        let structures: Vec<Box<dyn StructurePlacer + Send + Sync>> =
            vec![Box::new(Village), Box::new(Temple)];
        Self {
            _seed: seed,
            noise_generator,
            structures,
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

    fn apply_surface(&self, _chunk: &mut Chunk) {}

    fn apply_carvers(&self, _chunk: &mut Chunk) {}

    fn apply_features(&self, _chunk: &mut Chunk) {}

    fn apply_structures(&self, chunk: &mut Chunk) {
        for s in &self.structures {
            s.place(chunk);
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let id = self.biome_at(x, z);
        let mut chunk = self.generate_chunk_for_biome(x, z, id)?;
        self.apply_surface(&mut chunk);
        self.apply_carvers(&mut chunk);
        self.apply_features(&mut chunk);
        self.apply_structures(&mut chunk);
        Ok(chunk)
    }
}
