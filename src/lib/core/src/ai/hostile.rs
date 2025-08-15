use super::{AIGoal, EntityKind};
use crate::transform::position::Position;

const HOSTILE_MOBS: &[EntityKind] = &[
    EntityKind::Blaze,
    EntityKind::CaveSpider,
    EntityKind::Creeper,
    EntityKind::Drowned,
    EntityKind::Endermite,
    EntityKind::Evoker,
    EntityKind::Ghast,
    EntityKind::Guardian,
    EntityKind::Hoglin,
    EntityKind::Husk,
    EntityKind::Illusioner,
    EntityKind::MagmaCube,
    EntityKind::Phantom,
    EntityKind::PiglinBrute,
    EntityKind::Pillager,
    EntityKind::Ravager,
    EntityKind::Shulker,
    EntityKind::Silverfish,
    EntityKind::Skeleton,
    EntityKind::SkeletonHorse,
    EntityKind::Slime,
    EntityKind::Spider,
    EntityKind::Stray,
    EntityKind::Vex,
    EntityKind::Vindicator,
    EntityKind::Witch,
    EntityKind::WitherSkeleton,
    EntityKind::Zoglin,
    EntityKind::Zombie,
    EntityKind::ZombieHorse,
    EntityKind::ZombieVillager,
    EntityKind::Giant,
    EntityKind::ElderGuardian,
    EntityKind::EnderDragon,
    EntityKind::Warden,
    EntityKind::Wither,
];

pub fn goals(kind: EntityKind) -> Option<Vec<AIGoal>> {
    if HOSTILE_MOBS.contains(&kind) {
        Some(vec![
            AIGoal::Target { target: Position::default() },
            AIGoal::Attack { target: Position::default() },
        ])
    } else {
        None
    }
}

