use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Component, Debug, Clone)]
pub struct Zombie;

#[derive(TypeName, Component, Debug, Clone)]
pub struct Skeleton;

#[derive(TypeName, Component, Debug, Clone)]
pub struct Cow;

pub mod spawn_rules;
