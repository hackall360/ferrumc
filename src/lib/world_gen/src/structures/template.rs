use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use tracing::warn;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}

impl BoundingBox {
    fn new() -> Self {
        BoundingBox {
            min_x: i32::MAX,
            min_y: i32::MAX,
            min_z: i32::MAX,
            max_x: i32::MIN,
            max_y: i32::MIN,
            max_z: i32::MIN,
        }
    }

    fn update(&mut self, x: i32, y: i32, z: i32) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);
    }
}

#[derive(Deserialize)]
struct BlockDef {
    x: i32,
    y: i32,
    z: i32,
    block: String,
}

#[derive(Deserialize)]
struct TemplateDef {
    blocks: Vec<BlockDef>,
}

pub struct StructureTemplate {
    blocks: Vec<BlockDef>,
    bounding_box: BoundingBox,
}

impl StructureTemplate {
    pub fn from_json(data: &[u8]) -> Self {
        let def: TemplateDef =
            serde_json::from_slice(data).expect("structure template must be valid json");
        let mut bb = BoundingBox::new();
        for b in &def.blocks {
            bb.update(b.x, b.y, b.z);
        }
        StructureTemplate {
            blocks: def.blocks,
            bounding_box: bb,
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    pub fn place_randomized(&self, chunk: &mut Chunk, seed: u64, origin_y: i32) {
        let chunk_origin_x = chunk.x * 16;
        let chunk_origin_z = chunk.z * 16;
        let mut rng =
            rand::rngs::StdRng::seed_from_u64(seed ^ ((chunk.x as u64) << 32) ^ chunk.z as u64);
        let rotation = rng.random_range(0..4) as u8;

        for block in &self.blocks {
            let (rx, rz) = rotate(block.x, block.z, rotation);
            let world_x = chunk_origin_x + rx;
            let world_y = origin_y + block.y;
            let world_z = chunk_origin_z + rz;

            if world_x < chunk_origin_x
                || world_x >= chunk_origin_x + 16
                || world_z < chunk_origin_z
                || world_z >= chunk_origin_z + 16
            {
                continue;
            }

            let block_data = BlockData {
                name: block.block.clone(),
                properties: None,
            };
            if let Err(e) = chunk.set_block(
                world_x - chunk_origin_x,
                world_y,
                world_z - chunk_origin_z,
                block_data.to_block_id(),
            ) {
                warn!("structure placement failed: {e}");
            }
        }
    }
}

fn rotate(x: i32, z: i32, rotation: u8) -> (i32, i32) {
    match rotation % 4 {
        0 => (x, z),
        1 => (-z, x),
        2 => (-x, -z),
        _ => (z, -x),
    }
}
