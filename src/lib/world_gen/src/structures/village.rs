use super::template::StructureTemplate;
use super::{StructurePlacer, should_place_structure};
use ferrumc_world::chunk_format::Chunk;
use once_cell::sync::Lazy;

static TEMPLATE: Lazy<StructureTemplate> =
    Lazy::new(|| StructureTemplate::from_json(include_bytes!("templates/village.json")));

pub struct Village;

impl StructurePlacer for Village {
    fn place(&self, chunk: &mut Chunk, seed: u64) {
        if !should_place_structure(seed, 10_387_312, 32, 8, chunk.x, chunk.z) {
            return;
        }
        TEMPLATE.place_randomized(chunk, seed, 64);
    }
}
