use bevy_ecs::prelude::{Component, Entity, Event, EventWriter};
use typename::TypeName;

/// Tracks a player's health, armor and regeneration rate in "hearts".
#[derive(TypeName, Component, Debug, Clone)]
pub struct Health {
    /// Current number of hearts the entity has.
    pub hearts: f32,
    /// Maximum hearts the entity can have.
    pub max_hearts: f32,
    /// Armor value used to mitigate incoming damage.
    pub armor: f32,
    /// Hearts regenerated per second.
    pub regen_rate: f32,
}

impl Health {
    /// Creates a new [`Health`] component with the given maximum hearts.
    pub fn new(max_hearts: f32) -> Self {
        Self {
            hearts: max_hearts,
            max_hearts,
            armor: 0.0,
            regen_rate: 0.0,
        }
    }

    /// Applies raw damage to the entity, reduced by its armor value.
    pub fn damage(&mut self, amount: f32) {
        let dmg = (amount - self.armor).max(0.0);
        self.hearts = (self.hearts - dmg).max(0.0);
    }

    /// Heals the entity by the given number of hearts.
    pub fn heal(&mut self, amount: f32) {
        self.hearts = (self.hearts + amount).min(self.max_hearts);
    }

    /// Regenerates health based on the configured regeneration rate and
    /// the elapsed time in seconds.
    pub fn regenerate(&mut self, delta: f32) {
        if self.hearts < self.max_hearts {
            self.hearts = (self.hearts + self.regen_rate * delta).min(self.max_hearts);
        }
    }

    /// Returns true if the entity's hearts have been depleted.
    pub fn is_dead(&self) -> bool {
        self.hearts <= 0.0
    }
}

/// Event fired when an entity's health changes and clients must be updated.
#[derive(Event, Debug, Clone)]
pub struct HealthChangeEvent {
    pub entity: Entity,
    pub hearts: f32,
    pub armor: f32,
}

/// Helper for emitting [`HealthChangeEvent`]s.
pub fn notify_health_change(
    writer: &mut EventWriter<HealthChangeEvent>,
    entity: Entity,
    hearts: f32,
    armor: f32,
) {
    writer.write(HealthChangeEvent {
        entity,
        hearts,
        armor,
    });
}
