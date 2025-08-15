use super::{AIGoal, EntityKind};
use crate::transform::position::Position;

const PASSIVE_MOBS: &[EntityKind] = &[
    EntityKind::Allay,
    EntityKind::Axolotl,
    EntityKind::Bat,
    EntityKind::Camel,
    EntityKind::Cat,
    EntityKind::Chicken,
    EntityKind::Cod,
    EntityKind::Cow,
    EntityKind::Dolphin,
    EntityKind::Donkey,
    EntityKind::Fox,
    EntityKind::Frog,
    EntityKind::GlowSquid,
    EntityKind::Horse,
    EntityKind::Mooshroom,
    EntityKind::Mule,
    EntityKind::Ocelot,
    EntityKind::Panda,
    EntityKind::Parrot,
    EntityKind::Pig,
    EntityKind::Rabbit,
    EntityKind::Salmon,
    EntityKind::Sheep,
    EntityKind::Sniffer,
    EntityKind::Squid,
    EntityKind::Strider,
    EntityKind::Tadpole,
    EntityKind::TropicalFish,
    EntityKind::Turtle,
    EntityKind::TraderLlama,
    EntityKind::Goat,
    EntityKind::Pufferfish,
];

pub fn goals(kind: EntityKind) -> Option<Vec<AIGoal>> {
    if PASSIVE_MOBS.contains(&kind) {
        Some(vec![
            AIGoal::Wander,
            AIGoal::Graze,
            AIGoal::Flee { from: Position::default() },
        ])
    } else {
        None
    }
}

