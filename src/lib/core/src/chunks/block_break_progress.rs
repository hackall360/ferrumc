use bevy_ecs::prelude::Resource;
use std::collections::HashMap;

/// Tracks block breaking progress for positions in a chunk.
#[derive(Resource, Default)]
pub struct BlockBreakProgress {
    progress: HashMap<(i32, i32, i32, String), f32>,
}

impl BlockBreakProgress {
    fn key(x: i32, y: i32, z: i32, dimension: &str) -> (i32, i32, i32, String) {
        (x, y, z, dimension.to_string())
    }

    /// Start tracking break progress for a block at the given position within the specified dimension.
    pub fn start(&mut self, x: i32, y: i32, z: i32, dimension: String) {
        self.progress
            .entry(Self::key(x, y, z, &dimension))
            .or_insert(0.0);
    }

    /// Update the progress for the block at the given position. Returns the new progress if tracked.
    pub fn update(&mut self, x: i32, y: i32, z: i32, dimension: &str, delta: f32) -> Option<f32> {
        self.progress
            .get_mut(&Self::key(x, y, z, dimension))
            .map(|p| {
                *p = (*p + delta).clamp(0.0, 1.0);
                *p
            })
    }

    /// Stop tracking the block at the given position.
    pub fn clear(&mut self, x: i32, y: i32, z: i32, dimension: &str) {
        self.progress.remove(&Self::key(x, y, z, dimension));
    }

    /// Fetch the current progress for the given block position.
    pub fn get(&self, x: i32, y: i32, z: i32, dimension: &str) -> Option<f32> {
        self.progress.get(&Self::key(x, y, z, dimension)).copied()
    }
}
