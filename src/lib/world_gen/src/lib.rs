mod biomes;
pub mod end;
pub mod errors;
pub mod nether;
mod noise_settings;
mod structures;

use crate::errors::WorldGenError;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use noise::{Clamp, NoiseFn, OpenSimplex};
use noise_settings::{NoiseSettings, OVERWORLD_NOISE_SETTINGS};
use std::collections::BTreeMap;
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
            let scale = self.settings.xz_factor * self.settings.xz_scale.powi(c as i32 + 1);
            noise += layer.get([x / scale, z / scale]);
        }
        noise / (self.layers.len() as f64)
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

    fn apply_surface(&self, chunk: &mut Chunk) {
        // Add bedrock barriers according to the dimension.
        let bedrock = BlockData {
            name: "minecraft:bedrock".into(),
            properties: None,
        };
        match chunk.dimension.as_str() {
            // Overworld and End have only a floor layer.
            "overworld" | "the_end" => {
                for x in 0..16 {
                    for z in 0..16 {
                        // Ignore errors from missing sections; generation proceeds regardless.
                        let _ = chunk.set_block(x, 0, z, bedrock.to_block_id());
                    }
                }
            }
            // Nether has both a floor and a ceiling layer of bedrock.
            "the_nether" => {
                let id = bedrock.to_block_id();
                for x in 0..16 {
                    for z in 0..16 {
                        let _ = chunk.set_block(x, 0, z, id);
                        let _ = chunk.set_block(x, 127, z, id);
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_carvers(&self, chunk: &mut Chunk) {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;
        let air = BlockData {
            name: "minecraft:air".into(),
            properties: None,
        };
        let air_id = air.to_block_id();
        match chunk.dimension.as_str() {
            "overworld" => {
                for lx in 0..16 {
                    for lz in 0..16 {
                        let global_x = chunk_x * 16 + lx;
                        let global_z = chunk_z * 16 + lz;
                        let n = self
                            .noise_generator
                            .get_noise(global_x as f64, global_z as f64);
                        if n > 0.65 {
                            for y in 20..50 {
                                // Ignore errors from sections that may not exist yet
                                let _ = chunk.set_block(lx, y, lz, air_id);
                            }
                        }
                    }
                }
            }
            "the_nether" => {
                for lx in 0..16 {
                    for lz in 0..16 {
                        let global_x = chunk_x * 16 + lx;
                        let global_z = chunk_z * 16 + lz;
                        let n = self
                            .noise_generator
                            .get_noise(global_x as f64, global_z as f64);
                        if n > 0.55 {
                            for y in 10..80 {
                                let _ = chunk.set_block(lx, y, lz, air_id);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn apply_features(&self, chunk: &mut Chunk) {
        match chunk.dimension.as_str() {
            "overworld" => {
                // Fill oceans with water up to sea level
                let water = BlockData {
                    name: "minecraft:water".to_string(),
                    properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
                };
                let water_id = water.to_block_id();
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..63 {
                            if let Ok(block) = chunk.get_block(x, y, z) {
                                if block == BlockId::default() {
                                    let _ = chunk.set_block(x, y, z, water_id);
                                }
                            }
                        }
                    }
                }
            }
            "the_nether" => {
                // Add a lava ocean similar to vanilla behaviour
                let lava = BlockData {
                    name: "minecraft:lava".to_string(),
                    properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
                };
                let lava_id = lava.to_block_id();
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..32 {
                            if let Ok(block) = chunk.get_block(x, y, z) {
                                if block == BlockId::default() {
                                    let _ = chunk.set_block(x, y, z, lava_id);
                                }
                            }
                        }
                    }
                }
            }
            "the_end" => {
                // The End uses no additional fluid features.
            }
            _ => {}
        }
    }

    fn apply_structures(&self, chunk: &mut Chunk) {
        for s in &self.structures {
            s.place(chunk, self._seed);
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
