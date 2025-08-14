pub trait StructurePlacer {
    /// Attempts to place the structure in the provided chunk.
    ///
    /// The `seed` is used for deterministic placement following the
    /// vanilla structure spacing algorithm.
    fn place(&self, chunk: &mut ferrumc_world::chunk_format::Chunk, seed: u64);
}

pub mod template;
pub mod temple;
pub mod village;

use rand::{Rng, SeedableRng};

fn floor_div(a: i32, b: i32) -> i32 {
    let mut r = a / b;
    if (a ^ b) < 0 && a % b != 0 {
        r -= 1;
    }
    r
}

/// Determines whether a structure should attempt placement in the given chunk
/// using the vanilla spacing and separation algorithm.
pub fn should_place_structure(
    seed: u64,
    salt: u64,
    spacing: i32,
    separation: i32,
    chunk_x: i32,
    chunk_z: i32,
) -> bool {
    let region_x = floor_div(chunk_x, spacing);
    let region_z = floor_div(chunk_z, spacing);
    let mut rng = rand::rngs::StdRng::seed_from_u64(
        (region_x as i64 * 341_873_128_712
            + region_z as i64 * 132_897_987_541
            + seed as i64
            + salt as i64) as u64,
    );
    let offset_x = rng.random_range(0..(spacing - separation));
    let offset_z = rng.random_range(0..(spacing - separation));
    let candidate_x = region_x * spacing + offset_x;
    let candidate_z = region_z * spacing + offset_z;
    chunk_x == candidate_x && chunk_z == candidate_z
}
