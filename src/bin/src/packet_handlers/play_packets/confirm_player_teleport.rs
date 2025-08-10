use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::pending_teleport::PendingTeleport;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::ConfirmPlayerTeleportReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{error, warn};

pub fn handle(
    events: Res<ConfirmPlayerTeleportReceiver>,
    mut commands: Commands,
    mut pos_query: Query<(&mut Position, &mut OnGround, &Rotation, &PlayerIdentity)>,
    pending_query: Query<&PendingTeleport>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in events.0.try_iter() {
        if !state.0.players.is_connected(eid) {
            warn!("Entity {:?} is not connected, cannot confirm teleport", eid);
            continue;
        }

        let Ok(pending) = pending_query.get(eid) else {
            warn!("No pending teleport for entity {:?}", eid);
            continue;
        };

        if event.teleport_id.0 != pending.id {
            warn!(
                "Teleport ID mismatch for entity {:?}: expected {}, got {}",
                eid, pending.id, event.teleport_id.0
            );
            continue;
        }

        if let Ok((mut pos, mut on_ground, rot, identity)) = pos_query.get_mut(eid) {
            *pos = Position::new(pending.position.x, pending.position.y, pending.position.z);
            on_ground.0 = false;
            commands.entity(eid).remove::<PendingTeleport>();

            let packet = TeleportEntityPacket::new(identity.short_uuid, &pos, rot, on_ground.0);
            for (entity, conn) in conn_query.iter() {
                if entity == eid || !state.0.players.is_connected(entity) {
                    continue;
                }
                if let Err(err) = conn.send_packet_ref(&packet) {
                    error!(
                        "Failed to send teleport entity packet to {:?}: {:?}",
                        entity, err
                    );
                }
            }
        } else {
            warn!("Failed to get position components for entity {:?}", eid);
        }
    }
}
