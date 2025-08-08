use bevy_ecs::prelude::{Entity, Event};

#[derive(Event)]
pub struct ChangeDimensionEvent {
    pub player: Entity,
    pub dimension: String,
}
