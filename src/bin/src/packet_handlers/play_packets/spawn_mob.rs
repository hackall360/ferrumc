use bevy_ecs::prelude::{Commands, Entity, Query, With};
use ferrumc_core::ai::{Mob, PendingSpawn};
use ferrumc_core::identity::entity_id::EntityId;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::warn;

pub fn handle(
    mut commands: Commands,
    mobs: Query<(Entity, &EntityId, &Mob, &Position, &Rotation), With<PendingSpawn>>,
    connections: Query<&StreamWriter>,
) {
    for (entity, id, mob, pos, rot) in mobs.iter() {
        let packet = SpawnEntityPacket::mob(id, mob.kind, pos, rot);
        for conn in connections.iter() {
            if let Err(e) = conn.send_packet_ref(&packet) {
                warn!("Failed to send spawn packet: {:?}", e);
            }
        }
        commands.entity(entity).remove::<PendingSpawn>();
    }
}
