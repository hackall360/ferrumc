use bevy_ecs::prelude::Component;
use typename::TypeName;

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
