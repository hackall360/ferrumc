use super::StructurePlacer;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;

pub struct Temple;

impl StructurePlacer for Temple {
    fn place(&self, chunk: &mut Chunk) {
        let block = BlockData {
            name: "minecraft:mossy_cobblestone".into(),
            properties: None,
        };
        // Ignore placement errors; this implementation is a simplified placeholder.
        let _ = chunk.set_block(1, 64, 1, block.to_block_id());
    }
}
