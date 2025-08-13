use super::{should_place_structure, StructurePlacer};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;

pub struct Village;

impl StructurePlacer for Village {
    fn place(&self, chunk: &mut Chunk, seed: u64) {
        if !should_place_structure(seed, 10_387_312, 32, 8, chunk.x, chunk.z) {
            return;
        }
        let block = BlockData {
            name: "minecraft:cobblestone".into(),
            properties: None,
        };
        // Placement failures are ignored for now as the structure system is placeholder-only.
        let _ = chunk.set_block(0, 64, 0, block.to_block_id());
    }
}
