use ferrumc_core::ai::EntityKind;
use ferrumc_core::attributes::attributes_for;

#[test]
fn attribute_initialization_matches_expected() {
    let player = attributes_for(EntityKind::Player);
    assert_eq!(player.health.hearts, 20.0);
    assert_eq!(player.health.armor, 0.0);
    assert_eq!(player.health.regen_rate, 0.0);
    assert!((player.speed.0 - 0.1).abs() < f64::EPSILON);

    let cow = attributes_for(EntityKind::Cow);
    assert_eq!(cow.health.hearts, 20.0);
    assert_eq!(cow.health.armor, 0.0);
    assert_eq!(cow.health.regen_rate, 0.0);
    assert!((cow.speed.0 - 0.2).abs() < f64::EPSILON);

    let zombie = attributes_for(EntityKind::Zombie);
    assert_eq!(zombie.health.hearts, 20.0);
    assert_eq!(zombie.health.armor, 2.0);
    assert_eq!(zombie.health.regen_rate, 0.0);
    assert!((zombie.speed.0 - 0.23).abs() < f64::EPSILON);

    let skeleton = attributes_for(EntityKind::Skeleton);
    assert_eq!(skeleton.health.hearts, 20.0);
    assert_eq!(skeleton.health.armor, 2.0);
    assert_eq!(skeleton.health.regen_rate, 0.0);
    assert!((skeleton.speed.0 - 0.25).abs() < f64::EPSILON);
}
