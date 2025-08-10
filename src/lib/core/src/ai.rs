use bevy_ecs::prelude::Component;
use typename::TypeName;

use ferrumc_macros::get_registry_entry;

#[derive(TypeName, Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityKind {
    Cow,
}

const COW_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cow");

impl EntityKind {
    pub fn network_id(self) -> i32 {
        match self {
            EntityKind::Cow => COW_ID as i32,
        }
    }
}

#[derive(TypeName, Component, Debug, Clone)]
pub struct Mob {
    pub kind: EntityKind,
}

#[derive(TypeName, Component, Debug, Default)]
pub struct PendingSpawn;

#[derive(TypeName, Component, Debug, Clone, Eq, PartialEq)]
pub enum AIGoal {
    Idle,
    Wander,
}

impl Default for AIGoal {
    fn default() -> Self {
        AIGoal::Idle
    }
}
