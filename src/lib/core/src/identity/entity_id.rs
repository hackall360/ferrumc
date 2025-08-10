use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Debug, Component, Clone)]
pub struct EntityId {
    pub uuid: uuid::Uuid,
    pub short_uuid: i32,
}

impl EntityId {
    pub fn new(uuid: u128) -> Self {
        Self {
            uuid: uuid::Uuid::from_u128(uuid),
            short_uuid: uuid as i32,
        }
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new(0)
    }
}
