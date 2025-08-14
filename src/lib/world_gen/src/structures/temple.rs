use super::template::StructureTemplate;
use super::{StructurePlacer, should_place_structure};
use ferrumc_world::chunk_format::Chunk;
use once_cell::sync::Lazy;

static TEMPLATE: Lazy<StructureTemplate> =
    Lazy::new(|| StructureTemplate::from_json(include_bytes!("templates/temple.json")));

pub struct Temple;

impl StructurePlacer for Temple {
    fn place(&self, chunk: &mut Chunk, seed: u64) {
        if !should_place_structure(seed, 14_357_617, 32, 8, chunk.x, chunk.z) {
            return;
        }
        TEMPLATE.place_randomized(chunk, seed, 64);
    }
}
