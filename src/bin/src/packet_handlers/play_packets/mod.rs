use crate::systems::chat_message;
use bevy_ecs::schedule::Schedule;

mod chunk_batch_ack;
mod confirm_player_teleport;
mod keep_alive;
mod place_block;
mod player_action;
mod player_command;
mod container_slot_state_changed;
mod container_close;
mod player_loaded;
mod chat_ack;
mod client_information;
mod set_player_position;
mod set_player_position_and_rotation;
mod set_player_rotation;
mod swing_arm;
mod spawn_entity;
mod despawn_entity;

pub fn register_packet_handlers(schedule: &mut Schedule) {
    // Added separately so if we mess up the signature of one of the systems we can know exactly
    // which one
    schedule.add_systems(chunk_batch_ack::handle);
    schedule.add_systems(confirm_player_teleport::handle);
    schedule.add_systems(keep_alive::handle);
    schedule.add_systems(place_block::handle);
    schedule.add_systems(player_action::handle);
    schedule.add_systems(player_command::handle);
    schedule.add_systems(container_slot_state_changed::handle);
    schedule.add_systems(container_close::handle);
    schedule.add_systems(chat_message::broadcast_chat_messages);
    schedule.add_systems(chat_ack::handle);
    schedule.add_systems(client_information::handle);
    schedule.add_systems(set_player_position::handle);
    schedule.add_systems(set_player_position_and_rotation::handle);
    schedule.add_systems(set_player_rotation::handle);
    schedule.add_systems(swing_arm::handle);
    schedule.add_systems(player_loaded::handle);
    schedule.add_systems(spawn_entity::handle);
    schedule.add_systems(despawn_entity::handle);
}
