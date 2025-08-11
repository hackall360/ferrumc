use crate::collisions::bounds::CollisionBounds;
use ferrumc_world::{block_id::BlockId, chunk_format::Chunk};

/// Query the collision bounds for a block within the given chunk.
///
/// Blocks that are considered non-solid (such as air or water) will
/// return `None`, indicating no collision shape.
pub fn block_bounds(chunk: &Chunk, x: i32, y: i32, z: i32) -> Option<CollisionBounds> {
    // If the chunk lookup fails, treat as empty space.
    let block = chunk.get_block(x, y, z).ok()?;
    if block == BlockId::default() {
        return None;
    }
    let data = block.to_block_data().unwrap_or_default();
    // Treat water and other fluids as non-solid for collision purposes.
    if data.name.contains("water") || data.name.contains("lava") {
        return None;
    }
    Some(CollisionBounds {
        x_offset_start: 0.0,
        x_offset_end: 1.0,
        y_offset_start: 0.0,
        y_offset_end: 1.0,
        z_offset_start: 0.0,
        z_offset_end: 1.0,
    })
}
