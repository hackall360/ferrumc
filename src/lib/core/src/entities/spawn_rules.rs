use crate::ai::EntityKind;

pub struct SpawnRule {
    pub kind: EntityKind,
    pub weight: u32,
}

pub fn rules_for_biome(biome: u8) -> &'static [SpawnRule] {
    match biome {
        39 => PLAINS,
        _ => DEFAULT,
    }
}

static PLAINS: &[SpawnRule] = &[
    SpawnRule {
        kind: EntityKind::Zombie,
        weight: 50,
    },
    SpawnRule {
        kind: EntityKind::Skeleton,
        weight: 50,
    },
];

static DEFAULT: &[SpawnRule] = &[SpawnRule {
    kind: EntityKind::Cow,
    weight: 100,
}];
