use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Component, Debug, Clone, Copy, Default)]
pub struct Movement {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

impl Movement {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self { vx, vy, vz }
    }
}
