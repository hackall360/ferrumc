use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::{IncomingChatMessagePacketReceiver, connection::StreamWriter, packets::outgoing::chat_message::OutgoingChatMessagePacket};
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::error;

pub fn broadcast_chat_messages(
    events: Res<IncomingChatMessagePacketReceiver>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (packet, _) in events.0.try_iter() {
        let message = TextComponent::from(packet.message.clone());
        let outgoing = OutgoingChatMessagePacket::new(message);
        for (entity, conn) in query.iter() {
            if !state.0.players.is_connected(entity) {
                continue;
            }
            if let Err(err) = conn.send_packet_ref(&outgoing) {
                error!("Failed to send chat message: {:?}", err);
            }
        }
    }
}

