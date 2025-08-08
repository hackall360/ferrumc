use bevy_ecs::prelude::Query;
use ferrumc_core::ai::AIGoal;
use ferrumc_core::movement::Movement;

pub fn update_ai(mut query: Query<(&AIGoal, &mut Movement)>) {
    for (goal, mut movement) in query.iter_mut() {
        match goal {
            AIGoal::Idle => {
                movement.vx = 0.0;
                movement.vy = 0.0;
                movement.vz = 0.0;
            }
            AIGoal::Wander => {
                if movement.vx == 0.0 && movement.vz == 0.0 {
                    movement.vx = 0.1;
                }
            }
        }
    }
}
