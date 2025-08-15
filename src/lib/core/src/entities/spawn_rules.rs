use crate::ai::EntityKind;

/// Dimension an entity is allowed to spawn in.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

/// Describes the spawning conditions for an [`EntityKind`].
#[derive(Clone, Copy)]
pub struct SpawnRule {
    pub kind: EntityKind,
    pub weight: u32,
    /// Minimum light level required for spawning.
    pub min_light: Option<u8>,
    /// Maximum light level allowed for spawning.
    pub max_light: Option<u8>,
    /// Dimension restriction for the entity.
    pub dimension: Dimension,
}

/// Returns spawn rules for the given biome id.
pub fn rules_for_biome(biome: u8) -> &'static [SpawnRule] {
    match biome {
        // badlands/desert variants
        0 | 14 | 19 | 63 => DESERT_RULES,
        // jungles
        1 | 28 | 49 => JUNGLE_RULES,
        // basalt deltas
        2 => BASALT_DELTAS_RULES,
        // beaches
        3 | 44 | 51 => BEACH_RULES,
        // forests of all kinds
        4 | 5 | 8 | 20 | 21 | 36 | 59 => FOREST_RULES,
        // oceans
        6 | 9 | 11 | 12 | 13 | 22 | 29 | 35 | 57 => OCEAN_RULES,
        // crimson forest
        7 => CRIMSON_FOREST_RULES,
        // deep dark
        10 => DEEP_DARK_RULES,
        // deep/dripstone caves
        15 => DRIPSTONE_CAVES_RULES,
        // end biomes
        16 | 17 | 18 | 43 | 55 => END_RULES,
        // frozen peaks and mountain variants
        23 | 27 | 50 | 60 | 61 => MOUNTAIN_RULES,
        // snowy/icy areas
        25 | 26 | 45 | 46 => SNOW_RULES,
        // taiga variants
        37 | 38 | 47 | 54 => TAIGA_RULES,
        // jungle-like swamps
        31 | 53 => SWAMP_RULES,
        // meadow and plains variants
        32 | 39 | 52 => PLAINS_RULES,
        // mushroom fields
        33 => MUSHROOM_RULES,
        // nether wastes
        34 => NETHER_WASTES_RULES,
        // rivers
        40 | 24 => RIVER_RULES,
        // savanna variants
        41 | 42 | 62 => SAVANNA_RULES,
        // soul sand valley
        48 => SOUL_SAND_VALLEY_RULES,
        // lush caves
        30 => LUSH_CAVES_RULES,
        // mangrove swamp already covered
        // warm ocean handled in ocean group above
        // warped forest
        58 => WARPED_FOREST_RULES,
        // the void
        56 => VOID_RULES,
        // everything else defaults to plains
        _ => PLAINS_RULES,
    }
}

// ----- Overworld rule sets -----

static PLAINS_RULES: &[SpawnRule] = &[
    // passive animals
    SpawnRule { kind: EntityKind::Cow, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Sheep, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Pig, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Chicken, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    // hostiles at night
    SpawnRule { kind: EntityKind::Zombie, weight: 15, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 15, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Spider, weight: 10, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static FOREST_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Cow, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Pig, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Chicken, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Wolf, weight: 10, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 15, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 15, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Spider, weight: 10, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static JUNGLE_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Parrot, weight: 10, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Ocelot, weight: 10, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Chicken, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Spider, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static DESERT_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Rabbit, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Husk, weight: 60, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Spider, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static SAVANNA_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Cow, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Sheep, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Horse, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static TAIGA_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Wolf, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Fox, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Rabbit, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static SNOW_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::PolarBear, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Rabbit, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Stray, weight: 40, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static SWAMP_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Frog, weight: 20, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Slime, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static MOUNTAIN_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Goat, weight: 40, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 30, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Skeleton, weight: 30, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static BEACH_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Turtle, weight: 40, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 60, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static OCEAN_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Cod, weight: 30, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Salmon, weight: 30, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Squid, weight: 20, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Drowned, weight: 20, min_light: None, max_light: None, dimension: Dimension::Overworld },
];

static RIVER_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Salmon, weight: 50, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Drowned, weight: 50, min_light: None, max_light: None, dimension: Dimension::Overworld },
];

static MUSHROOM_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Mooshroom, weight: 100, min_light: Some(8), max_light: None, dimension: Dimension::Overworld },
];

static LUSH_CAVES_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Axolotl, weight: 40, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::GlowSquid, weight: 40, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 20, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static DRIPSTONE_CAVES_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Bat, weight: 70, min_light: None, max_light: None, dimension: Dimension::Overworld },
    SpawnRule { kind: EntityKind::Zombie, weight: 30, min_light: None, max_light: Some(7), dimension: Dimension::Overworld },
];

static DEEP_DARK_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Warden, weight: 1, min_light: None, max_light: Some(0), dimension: Dimension::Overworld },
];

// ----- Nether rule sets -----

static NETHER_WASTES_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::ZombifiedPiglin, weight: 60, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::Ghast, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::MagmaCube, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
];

static CRIMSON_FOREST_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Piglin, weight: 40, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::Hoglin, weight: 40, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::ZombifiedPiglin, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
];

static WARPED_FOREST_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Enderman, weight: 60, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::ZombifiedPiglin, weight: 40, min_light: None, max_light: None, dimension: Dimension::Nether },
];

static BASALT_DELTAS_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::MagmaCube, weight: 60, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::Ghast, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::ZombifiedPiglin, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
];

static SOUL_SAND_VALLEY_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Skeleton, weight: 50, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::Ghast, weight: 30, min_light: None, max_light: None, dimension: Dimension::Nether },
    SpawnRule { kind: EntityKind::ZombifiedPiglin, weight: 20, min_light: None, max_light: None, dimension: Dimension::Nether },
];

// ----- End rule sets -----

static END_RULES: &[SpawnRule] = &[
    SpawnRule { kind: EntityKind::Enderman, weight: 80, min_light: None, max_light: None, dimension: Dimension::End },
    SpawnRule { kind: EntityKind::Shulker, weight: 20, min_light: None, max_light: None, dimension: Dimension::End },
];

static VOID_RULES: &[SpawnRule] = &[];

