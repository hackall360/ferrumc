use ferrumc_core::ai::EntityKind;
use ferrumc_core::entities::spawn_rules::rules_for_biome;

fn kinds_for(biome: u8) -> Vec<EntityKind> {
    rules_for_biome(biome).iter().map(|r| r.kind).collect()
}

#[test]
fn desert_biome_rules() {
    let kinds = kinds_for(0);
    assert_eq!(
        kinds,
        vec![EntityKind::Rabbit, EntityKind::Husk, EntityKind::Spider]
    );
}

#[test]
fn forest_biome_rules() {
    let kinds = kinds_for(4);
    assert_eq!(
        kinds,
        vec![
            EntityKind::Cow,
            EntityKind::Pig,
            EntityKind::Chicken,
            EntityKind::Wolf,
            EntityKind::Zombie,
            EntityKind::Skeleton,
            EntityKind::Spider,
        ]
    );
}

#[test]
fn nether_wastes_rules() {
    let kinds = kinds_for(34);
    assert_eq!(
        kinds,
        vec![
            EntityKind::ZombifiedPiglin,
            EntityKind::Ghast,
            EntityKind::MagmaCube,
        ]
    );
}
