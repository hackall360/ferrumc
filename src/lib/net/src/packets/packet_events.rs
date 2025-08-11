use bevy_ecs::prelude::{Entity, Event};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Event, Debug)]
pub struct TransformEvent {
    pub entity: Entity,
    pub position: Option<Position>,
    pub rotation: Option<Rotation>,
    pub on_ground: Option<bool>,
}
impl TransformEvent {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            position: None,
            rotation: None,
            on_ground: None,
        }
    }
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn on_ground(mut self, on_ground: bool) -> Self {
        self.on_ground = Some(on_ground);
        self
    }
}

#[derive(Event, Debug)]
pub struct CraftRecipeRequestEvent {
    pub window_id: i8,
    pub recipe: String,
    pub make_all: bool,
}

#[derive(Event, Debug)]
pub struct RecipeBookEvent {
    pub book_id: VarInt,
    pub open: bool,
    pub filtering: bool,
}

#[derive(Event, Debug)]
pub struct DisplayedRecipeEvent {
    pub recipe: String,
}
