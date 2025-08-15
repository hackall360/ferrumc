use crate::{
    collisions::{block::block_bounds, bounds::CollisionBounds},
    movement::Movement,
    transform::position::Position,
};
use ferrumc_world::chunk_format::Chunk;

/// Iterate over all block coordinates overlapped by the given bounds at the
/// provided position.
pub fn for_each_block_in_bounds<F>(
    _chunk: &Chunk,
    pos: &Position,
    bounds: &CollisionBounds,
    mut f: F,
) where
    F: FnMut(i32, i32, i32),
{
    let x_start = (pos.x + bounds.x_offset_start).floor() as i32;
    let x_end = (pos.x + bounds.x_offset_end - f64::EPSILON).floor() as i32;
    let y_start = (pos.y + bounds.y_offset_start).floor() as i32;
    let y_end = (pos.y + bounds.y_offset_end - f64::EPSILON).floor() as i32;
    let z_start = (pos.z + bounds.z_offset_start).floor() as i32;
    let z_end = (pos.z + bounds.z_offset_end - f64::EPSILON).floor() as i32;

    for x in x_start..=x_end {
        for y in y_start..=y_end {
            for z in z_start..=z_end {
                f(x, y, z);
            }
        }
    }
}

fn first_colliding_block(
    chunk: &Chunk,
    pos: &Position,
    bounds: &CollisionBounds,
) -> Option<(i32, i32, i32, CollisionBounds)> {
    let mut result = None;
    for_each_block_in_bounds(chunk, pos, bounds, |x, y, z| {
        if result.is_none() {
            if let Some(block_box) = block_bounds(chunk, x, y, z) {
                if bounds.collides(
                    (pos.x, pos.y, pos.z),
                    &block_box,
                    (x as f64, y as f64, z as f64),
                ) {
                    result = Some((x, y, z, block_box));
                }
            }
        }
    });
    result
}

/// Resolve collisions between an entity and blocks in the given chunk.
/// Returns true if a downward collision occurred, indicating the entity
/// is on the ground.
pub fn resolve_block_collisions(
    pos: &mut Position,
    bounds: &CollisionBounds,
    movement: &mut Movement,
    chunk: &Chunk,
) -> bool {
    let mut on_ground = false;

    // X axis
    pos.x += movement.vx;
    if movement.vx != 0.0 {
        while let Some((bx, by, bz, bb)) = first_colliding_block(chunk, pos, bounds) {
            if movement.vx > 0.0 {
                pos.x = bx as f64 + bb.x_offset_start - bounds.x_offset_end;
            } else {
                pos.x = bx as f64 + bb.x_offset_end - bounds.x_offset_start;
            }
            movement.vx = 0.0;
        }
    }

    // Y axis
    pos.y += movement.vy;
    if movement.vy != 0.0 {
        while let Some((bx, by, bz, bb)) = first_colliding_block(chunk, pos, bounds) {
            if movement.vy > 0.0 {
                pos.y = by as f64 + bb.y_offset_start - bounds.y_offset_end;
            } else {
                pos.y = by as f64 + bb.y_offset_end - bounds.y_offset_start;
                on_ground = true;
            }
            movement.vy = 0.0;
        }
    }

    // Z axis
    pos.z += movement.vz;
    if movement.vz != 0.0 {
        while let Some((bx, by, bz, bb)) = first_colliding_block(chunk, pos, bounds) {
            if movement.vz > 0.0 {
                pos.z = bz as f64 + bb.z_offset_start - bounds.z_offset_end;
            } else {
                pos.z = bz as f64 + bb.z_offset_end - bounds.z_offset_start;
            }
            movement.vz = 0.0;
        }
    }

    on_ground
}
