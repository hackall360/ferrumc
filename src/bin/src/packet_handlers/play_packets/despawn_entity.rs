use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_state::GlobalStateResource;
use tracing::warn;

pub fn handle(
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &StreamWriter, &PlayerIdentity)>,
) {
    let mut pending = Vec::new();
    while let Some(item) = state.0.players.disconnection_queue.pop() {
        pending.push(item);
    }
    if pending.is_empty() {
        return;
    }
    for (entity, _reason) in &pending {
        let Ok((_, _, identity)) = query.get(*entity) else {
            warn!("Failed to get identity for despawning entity {:?}", entity);
            continue;
        };
        let packet = RemoveEntitiesPacket::from_entities([identity.clone()]);
        for (other_entity, conn, _) in query.iter() {
            if other_entity == *entity {
                continue;
            }
            if let Err(e) = conn.send_packet_ref(&packet) {
                warn!("Failed to send remove entities packet to {:?}: {:?}", other_entity, e);
            }
        }
    }
    for item in pending {
        state.0.players.disconnection_queue.push(item);
    }
}
