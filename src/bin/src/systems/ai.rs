use bevy_ecs::prelude::{Commands, Query};
use ferrumc_core::ai::{AIGoal, EntityKind, Mob, PendingSpawn};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::identity::entity_id::EntityId;
use ferrumc_core::movement::Movement;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use rand::Rng;

pub fn spawn_mobs(mut cmd: Commands, query: Query<&Mob>) {
    if query.is_empty() {
        let id = EntityId::new(rand::random::<u128>());
        cmd.spawn((
            id,
            Mob { kind: EntityKind::Cow },
            Position::default(),
            Rotation::default(),
            Movement::default(),
            CollisionBounds {
                x_offset_start: -0.3,
                x_offset_end: 0.3,
                y_offset_start: 0.0,
                y_offset_end: 1.8,
                z_offset_start: -0.3,
                z_offset_end: 0.3,
            },
            AIGoal::Wander,
            PendingSpawn,
        ));
    }
}

pub fn update_ai(mut query: Query<(&AIGoal, &mut Movement)>) {
    for (goal, mut movement) in query.iter_mut() {
        match goal {
            AIGoal::Idle => {
                movement.vx = 0.0;
                movement.vy = 0.0;
                movement.vz = 0.0;
            }
            AIGoal::Wander => {
                let mut rng = rand::thread_rng();
                if movement.vx == 0.0 && movement.vz == 0.0 {
                    movement.vx = 0.1;
                }
                movement.vx += rng.gen_range(-0.05..0.05);
                movement.vz += rng.gen_range(-0.05..0.05);
            }
        }
    }
}
