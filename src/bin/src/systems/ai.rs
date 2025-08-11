use bevy_ecs::prelude::{Commands, Query, Res};
use ferrumc_core::ai::{AIGoal, Mob};
use ferrumc_core::movement::Movement;
use ferrumc_core::state::spawn_entities_from_biomes;
use ferrumc_state::GlobalStateResource;
use rand::Rng;

pub fn spawn_mobs(cmd: Commands, state: Res<GlobalStateResource>, query: Query<&Mob>) {
    spawn_entities_from_biomes(cmd, state, query);
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
