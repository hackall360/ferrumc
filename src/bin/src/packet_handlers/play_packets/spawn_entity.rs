use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use ferrumc_net::PlayerLoadedReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::warn;

pub fn handle(
    events: Res<PlayerLoadedReceiver>,
    state: Res<GlobalStateResource>,
    player_comps: Query<(&PlayerIdentity, &Position, &Rotation)>,
    connections: Query<(Entity, &StreamWriter)>,
) {
    for (_, entity) in events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            warn!("Entity {:?} is not connected, cannot broadcast spawn", entity);
            continue;
        }

        let Ok(new_player_packet) = SpawnEntityPacket::player(entity, player_comps) else {
            warn!("Failed to build spawn packet for entity {:?}", entity);
            continue;
        };

        for (other_entity, conn) in connections.iter() {
            if other_entity == entity {
                continue;
            }
            if let Err(e) = conn.send_packet_ref(&new_player_packet) {
                warn!("Failed to send spawn packet to {:?}: {:?}", other_entity, e);
            }
        }

        let Ok((_, new_conn)) = connections.get(entity) else {
            warn!("Failed to get connection for new entity {:?}", entity);
            continue;
        };
        for (other_entity, _) in connections.iter() {
            if other_entity == entity {
                continue;
            }
            if let Ok(packet) = SpawnEntityPacket::player(other_entity, player_comps) {
                if let Err(e) = new_conn.send_packet_ref(&packet) {
                    warn!(
                        "Failed to send existing entity {:?} to new player {:?}: {:?}",
                        other_entity, entity, e
                    );
                }
            }
        }
    }
}
