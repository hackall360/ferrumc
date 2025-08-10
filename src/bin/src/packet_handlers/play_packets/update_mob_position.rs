use bevy_ecs::prelude::{Query, With};
use ferrumc_core::ai::Mob;
use ferrumc_core::identity::entity_id::EntityId;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use tracing::warn;

pub fn handle(
    mobs: Query<(&EntityId, &Position, &Rotation), With<Mob>>,
    connections: Query<&StreamWriter>,
) {
    for (id, pos, rot) in mobs.iter() {
        let packet = TeleportEntityPacket::new(id.short_uuid, pos, rot, true);
        for conn in connections.iter() {
            if let Err(e) = conn.send_packet_ref(&packet) {
                warn!("Failed to send position packet: {:?}", e);
            }
        }
    }
}
