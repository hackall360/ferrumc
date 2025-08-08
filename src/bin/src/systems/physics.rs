use bevy_ecs::prelude::Query;
use ferrumc_core::movement::Movement;
use ferrumc_core::transform::position::Position;

pub fn update_physics(mut query: Query<(&mut Position, &Movement)>) {
    for (mut pos, movement) in query.iter_mut() {
        pos.x += movement.vx;
        pos.y += movement.vy;
        pos.z += movement.vz;
    }
}
