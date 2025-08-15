# Entity Registration and AI Overview

FerrumC models every creature, projectile, and utility object as an **entity**. This document outlines how
entities are registered, how their attributes and goals are configured, and how spawn rules
control where they appear.

## Registering entities

Entities are declared in [`src/lib/core/src/entities/mod.rs`](../src/lib/core/src/entities/mod.rs)
using the `entities!` macro. The macro expands into a `Component` type for each entity and makes it
available to Bevy's ECS:

```rust
entities!(
    Cow,
    Sheep,
    Zombie,
    // ...
);
```

Each registered type corresponds to an [`EntityKind`](../src/lib/core/src/ai.rs) value, which is used
when spawning and for network IDs.

## Attributes

Default health and movement speed are provided by [`attributes_for`](../src/lib/core/src/attributes.rs):

```rust
pub fn attributes_for(kind: EntityKind) -> Attributes {
    match kind {
        EntityKind::Player => Attributes {
            health: Health::with_attributes(20.0, 0.0, 0.0),
            speed: Speed(0.1),
        },
        EntityKind::Zombie => Attributes {
            health: Health::with_attributes(20.0, 2.0, 0.0),
            speed: Speed(0.23),
        },
        _ => Attributes {
            health: Health::with_attributes(20.0, 0.0, 0.0),
            speed: Speed::default(),
        },
    }
}
```

Unmapped entities fall back to a sensible default.

## Goals and AI

AI goals are grouped by behaviour class. Each module lists supported entities and returns a set of
[`AIGoal`](../src/lib/core/src/ai.rs) values. For example, passive mobs wander, graze, and flee:

```rust
const PASSIVE_MOBS: &[EntityKind] = &[EntityKind::Cow, EntityKind::Sheep, /* … */];

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
```

Similar modules exist for neutral and hostile behaviour classes.

## Spawn rules

[`SpawnRule`](../src/lib/core/src/entities/spawn_rules.rs) structures describe where an entity can appear:

```rust
SpawnRule {
    kind: EntityKind::Cow,
    weight: 20,
    min_light: Some(8),
    max_light: None,
    dimension: Dimension::Overworld,
}
```

`rules_for_biome` maps biome IDs to rule sets, allowing weighted random selection during world
ticks.

## Extending the system

To add a new entity:

1. **Register the type** in `entities/mod.rs` and add a matching `EntityKind` variant.
2. **Define attributes** in `attributes.rs` if custom health or speed is needed.
3. **Assign AI goals** by listing the entity in `ai/passive.rs`, `ai/neutral.rs`, or `ai/hostile.rs`.
4. **Create spawn rules** in `entities/spawn_rules.rs` for relevant biomes.

To tweak behaviour, modify the appropriate AI module. For instance, to make `Cow` entities
wander more aggressively, replace `AIGoal::Graze` with a custom goal and update the goal list.

This modular structure makes it straightforward to introduce new entities or experiment with
custom AI behaviour while keeping the rest of the system untouched.

