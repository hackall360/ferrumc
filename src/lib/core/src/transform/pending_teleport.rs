use bevy_ecs::prelude::Component;
use typename::TypeName;

use super::position::Position;

#[derive(TypeName, Component)]
pub struct PendingTeleport {
    pub id: i32,
    pub position: Position,
}

impl PendingTeleport {
    pub fn new(id: i32, position: Position) -> Self {
        Self { id, position }
    }
}
