pub trait StructurePlacer {
    fn place(&self, chunk: &mut ferrumc_world::chunk_format::Chunk);
}

pub mod temple;
pub mod village;
