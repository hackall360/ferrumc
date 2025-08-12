use bevy_ecs::event::Events;
use bevy_ecs::system::RunSystemOnce;
use bevy_ecs::world::World;
use ferrumc_core::combat::{handle_attacks, AttackEvent, DamageSource};
use ferrumc_core::health::{Health, HealthChangeEvent};

#[test]
fn combat_attack_reduces_health() {
    let mut world = World::new();
    world.insert_resource(Events::<AttackEvent>::default());
    world.insert_resource(Events::<HealthChangeEvent>::default());
    let attacker = world.spawn_empty().id();
    let victim = world
        .spawn(Health {
            hearts: 20.0,
            max_hearts: 20.0,
            armor: 0.0,
            regen_rate: 0.0,
        })
        .id();

    {
        let mut events = world.resource_mut::<Events<AttackEvent>>();
        events.send(AttackEvent {
            attacker,
            victim,
            amount: 5.0,
            source: DamageSource::Player(attacker),
        });
    }

    let _ = world.run_system_once(handle_attacks);
    let health = world.get::<Health>(victim).unwrap();
    assert_eq!(health.hearts, 15.0);
}
