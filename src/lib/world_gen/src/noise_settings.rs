/// Noise configuration values matching the official Minecraft 1.20.1 settings.
///
/// Only a small subset of the full vanilla data is represented here but these
/// values mirror the canonical datapack to give generators predictable output.
#[derive(Clone, Copy)]
pub struct NoiseSettings {
    /// Minimum Y coordinate of the noise grid.
    pub min_y: i32,
    /// Total vertical height of the noise grid.
    pub height: i32,
    /// Horizontal noise sampling scale.
    pub xz_scale: f64,
    /// Vertical noise sampling scale.
    pub y_scale: f64,
    /// Horizontal factor applied to the noise coordinates.
    pub xz_factor: f64,
    /// Vertical factor applied to the noise coordinates.
    pub y_factor: f64,
}

/// Overworld noise settings copied from the 1.20.1 data pack
pub const OVERWORLD_NOISE_SETTINGS: NoiseSettings = NoiseSettings {
    min_y: -64,
    height: 384,
    xz_scale: 1.0,
    y_scale: 1.0,
    xz_factor: 80.0,
    y_factor: 160.0,
};

/// Nether noise settings copied from the 1.20.1 data pack
pub const NETHER_NOISE_SETTINGS: NoiseSettings = NoiseSettings {
    min_y: 0,
    height: 128,
    xz_scale: 1.0,
    y_scale: 3.0,
    xz_factor: 80.0,
    y_factor: 60.0,
};

/// End noise settings copied from the 1.20.1 data pack
pub const END_NOISE_SETTINGS: NoiseSettings = NoiseSettings {
    min_y: 0,
    height: 128,
    xz_scale: 2.0,
    y_scale: 1.0,
    xz_factor: 80.0,
    y_factor: 160.0,
};
