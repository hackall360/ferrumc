use bevy_ecs::prelude::{Component, Entity, Event, EventWriter};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use typename::TypeName;

/// All known status effects in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TypeName)]
pub enum StatusEffect {
    Speed,
    Slowness,
    Regeneration,
    Strength,
}

/// Registry mapping string identifiers to [`StatusEffect`] values.
pub static STATUS_EFFECT_REGISTRY: Lazy<HashMap<&'static str, StatusEffect>> = Lazy::new(|| {
    use StatusEffect::*;
    HashMap::from([
        ("minecraft:speed", Speed),
        ("minecraft:slowness", Slowness),
        ("minecraft:regeneration", Regeneration),
        ("minecraft:strength", Strength),
    ])
});

impl StatusEffect {
    /// Returns the numeric network identifier used by the protocol for this effect.
    pub fn id(&self) -> i32 {
        match self {
            StatusEffect::Speed => 1,
            StatusEffect::Slowness => 2,
            StatusEffect::Regeneration => 10,
            StatusEffect::Strength => 5,
        }
    }
}

/// A single potion or status effect instance.
#[derive(Debug, Clone)]
pub struct Potion {
    pub effect: StatusEffect,
    pub amplifier: i32,
    pub duration: i32,
}

/// Component storing all active effects on an entity.
#[derive(Component, Debug, Default, Clone)]
pub struct ActiveEffects(pub Vec<Potion>);

/// Event fired when an effect is added to an entity.
#[derive(Event, Debug, Clone)]
pub struct EffectAddEvent {
    pub entity: Entity,
    pub effect: Potion,
}

/// Event fired when an effect is removed from an entity.
#[derive(Event, Debug, Clone)]
pub struct EffectRemoveEvent {
    pub entity: Entity,
    pub effect: StatusEffect,
}

/// Helper to emit an [`EffectAddEvent`].
pub fn add_effect(writer: &mut EventWriter<EffectAddEvent>, entity: Entity, effect: Potion) {
    writer.write(EffectAddEvent { entity, effect });
}

/// Helper to emit an [`EffectRemoveEvent`].
pub fn remove_effect(
    writer: &mut EventWriter<EffectRemoveEvent>,
    entity: Entity,
    effect: StatusEffect,
) {
    writer.write(EffectRemoveEvent { entity, effect });
}
