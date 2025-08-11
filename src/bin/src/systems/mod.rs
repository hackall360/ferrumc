mod ai;
pub mod chat_message;
pub mod connection_killer;
mod cross_chunk_boundary;
mod keep_alive_system;
pub mod new_connections;
mod physics;
mod player_count_update;
mod redstone_update;
pub mod send_chunks;
pub mod shutdown_systems;
mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(keep_alive_system::keep_alive_system);
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(cross_chunk_boundary::cross_chunk_boundary);
    schedule.add_systems(player_count_update::player_count_updater);
    schedule.add_systems(world_sync::sync_world);
    schedule.add_systems(ferrumc_core::state::tick_world);
      schedule.add_systems(ai::spawn_mobs);
      schedule.add_systems(ai::update_ai);
      schedule.add_systems(physics::update_physics);
    schedule.add_systems(redstone_update::run_redstone_updates);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}
