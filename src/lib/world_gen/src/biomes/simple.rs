use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use ferrumc_world::vanilla_chunk_format::BlockData;
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub(crate) enum Veg {
    None,
    Grass,
    OakTree,
    Cactus,
}

pub(crate) struct SimpleBiome {
    id: u8,
    name: &'static str,
    dimension: &'static str,
    stone: &'static str,
    surface: &'static str,
    filler: &'static str,
    veg: Veg,
    height_scale: f64,
}

impl SimpleBiome {
    pub(crate) fn new(
        id: u8,
        name: &'static str,
        dimension: &'static str,
        stone: &'static str,
        surface: &'static str,
        filler: &'static str,
        veg: Veg,
        height_scale: f64,
    ) -> Self {
        Self {
            id,
            name,
            dimension,
            stone,
            surface,
            filler,
            veg,
            height_scale,
        }
    }
}

impl BiomeGenerator for SimpleBiome {
    fn _biome_id(&self) -> u8 {
        self.id
    }

    fn _biome_name(&self) -> String {
        self.name.to_string()
    }

    fn generate_chunk(
        &self,
        x: i32,
        z: i32,
        noise: &NoiseGenerator,
    ) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, self.dimension.to_string());
        chunk.set_biome(self.id as i32);

        let mut heights = vec![];
        let stone = BlockData {
            name: self.stone.to_string(),
            properties: None,
        };

        // base fill
        let base_block = match self.dimension {
            "overworld" => BlockData {
                name: "minecraft:water".to_string(),
                properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
            },
            _ => BlockData {
                name: "minecraft:air".to_string(),
                properties: None,
            },
        };
        for section_y in -4..4 {
            chunk.set_section(section_y as i8, base_block.clone())?;
        }

        // heights
        for chunk_x in 0..16i64 {
            for chunk_z in 0..16i64 {
                let global_x = i64::from(x) * 16 + chunk_x;
                let global_z = i64::from(z) * 16 + chunk_z;
                let height = noise.get_noise(global_x as f64, global_z as f64);
                let height = (height * self.height_scale) as i32 + 64;
                heights.push((global_x, global_z, height));
            }
        }

        let y_min = heights.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
        let highest_full_section = y_min / 16;
        for section_y in -4..highest_full_section {
            chunk.set_section(section_y as i8, stone.clone())?;
        }
        let mut batch = EditBatch::new(&mut chunk);
        let above_filled_sections = (highest_full_section * 16) - 1;
        let surface_block = BlockData {
            name: self.surface.to_string(),
            properties: None,
        };
        let filler_block = BlockData {
            name: self.filler.to_string(),
            properties: None,
        };
        let mut tops = vec![];
        for (global_x, global_z, height) in heights {
            if height > above_filled_sections {
                let height = height - above_filled_sections;
                for y in 0..height {
                    let block = if y == height - 1 {
                        &surface_block
                    } else {
                        &filler_block
                    };
                    batch.set_block(
                        global_x as i32 & 0xF,
                        y + above_filled_sections,
                        global_z as i32 & 0xF,
                        block.clone(),
                    );
                }
                tops.push((global_x as i32 & 0xF, height + above_filled_sections, global_z as i32 & 0xF));
            }
        }

        // vegetation
        let mut rng = rand::rng();
        for (lx, top_y, lz) in tops {
            match self.veg {
                Veg::Grass => {
                    if rng.random::<f32>() < 0.2 {
                        batch.set_block(
                            lx,
                            top_y,
                            lz,
                            BlockData {
                                name: "minecraft:grass".to_string(),
                                properties: None,
                            },
                        );
                    }
                }
                Veg::OakTree => {
                    if rng.random::<f32>() < 0.05 {
                        for dy in 1..4 {
                            batch.set_block(
                                lx,
                                top_y + dy - 1,
                                lz,
                                BlockData {
                                    name: "minecraft:oak_log".to_string(),
                                    properties: None,
                                },
                            );
                        }
                        for dx in -1..=1 {
                            for dz in -1..=1 {
                                batch.set_block(
                                    lx + dx,
                                    top_y + 3,
                                    lz + dz,
                                    BlockData {
                                        name: "minecraft:oak_leaves".to_string(),
                                        properties: Some(BTreeMap::from([(
                                            "persistent".to_string(),
                                            "false".to_string(),
                                        )])),
                                    },
                                );
                            }
                        }
                    }
                }
                Veg::Cactus => {
                    if rng.random::<f32>() < 0.05 {
                        let h = rng.random_range(1..=3);
                        for dy in 0..h {
                            batch.set_block(
                                lx,
                                top_y + dy,
                                lz,
                                BlockData {
                                    name: "minecraft:cactus".to_string(),
                                    properties: None,
                                },
                            );
                        }
                    }
                }
                Veg::None => {}
            }
        }

        batch.apply()?;
        Ok(chunk)
    }
}
