use super::{AIGoal, EntityKind};
use crate::transform::position::Position;

const TRADERS: &[EntityKind] = &[
    EntityKind::Villager,
    EntityKind::WanderingTrader,
];

const DEFENDERS: &[EntityKind] = &[
    EntityKind::IronGolem,
    EntityKind::SnowGolem,
];

const TERRITORIAL: &[EntityKind] = &[
    EntityKind::Bee,
    EntityKind::Enderman,
    EntityKind::Llama,
    EntityKind::PolarBear,
    EntityKind::Piglin,
    EntityKind::ZombifiedPiglin,
    EntityKind::Wolf,
];

pub fn goals(kind: EntityKind) -> Option<Vec<AIGoal>> {
    let pos = Position::default();
    if TRADERS.contains(&kind) {
        return Some(vec![AIGoal::Trade, AIGoal::Flee { from: pos }]);
    }
    if DEFENDERS.contains(&kind) {
        return Some(vec![
            AIGoal::Defend { target: pos },
            AIGoal::Attack { target: pos },
        ]);
    }
    if TERRITORIAL.contains(&kind) {
        return Some(vec![
            AIGoal::Wander,
            AIGoal::Defend { target: pos },
        ]);
    }
    None
}

