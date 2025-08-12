use bevy_ecs::prelude::{Entity, Event, EventReader, EventWriter, Query};
use typename::TypeName;

use crate::health::{Health, HealthChangeEvent};

/// Sources that can inflict damage upon an entity.
#[derive(Debug, Clone, TypeName)]
pub enum DamageSource {
    Generic,
    Player(Entity),
    Mob(Entity),
    Fall,
}

/// Event fired when one entity attacks another.
#[derive(Event, Debug, Clone)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub victim: Entity,
    pub amount: f32,
    pub source: DamageSource,
}

/// System that applies [`AttackEvent`]s to entities with a [`Health`] component.
pub fn handle_attacks(
    mut attacks: EventReader<AttackEvent>,
    mut query: Query<&mut Health>,
    mut health_events: EventWriter<HealthChangeEvent>,
) {
    for atk in attacks.read() {
        if let Ok(mut health) = query.get_mut(atk.victim) {
            health.damage(atk.amount);
            health_events.write(HealthChangeEvent {
                entity: atk.victim,
                hearts: health.hearts,
                armor: health.armor,
            });
        }
    }
}
