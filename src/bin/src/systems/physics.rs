use bevy_ecs::prelude::{Entity, Query};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::movement::Movement;
use ferrumc_core::transform::position::Position;

pub fn update_physics(
    mut query: Query<(Entity, &mut Position, &Movement, &CollisionBounds)>,
) {
    let snapshot: Vec<(Entity, Position, CollisionBounds)> =
        query
            .iter()
            .map(|(e, pos, _, bounds)| (e, pos.clone(), *bounds))
            .collect();

    for (entity, mut pos, movement, bounds) in query.iter_mut() {
        let new_pos = Position::new(pos.x + movement.vx, pos.y + movement.vy, pos.z + movement.vz);
        let mut collided = false;
        for (other_e, other_pos, other_bounds) in snapshot.iter() {
            if entity == *other_e {
                continue;
            }
            if bounds.collides((new_pos.x, new_pos.y, new_pos.z), other_bounds, (other_pos.x, other_pos.y, other_pos.z)) {
                collided = true;
                break;
            }
        }
        if !collided {
            *pos = new_pos;
        }
    }
}
