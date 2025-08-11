/// Placeholder for Minecraft 1.20.1 noise settings.
///
/// These values do not fully replicate the vanilla implementation but
/// provide a hook for future expansion.
#[derive(Clone, Copy)]
pub struct NoiseSettings {
    pub scale: f64,
}

pub const OVERWORLD_NOISE_SETTINGS: NoiseSettings = NoiseSettings { scale: 64.0 };
