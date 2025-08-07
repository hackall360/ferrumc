use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::chat_message::OutgoingChatMessagePacket,
    IncomingChatMessagePacketReceiver,
};
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::error;

use crate::commands::{CommandContext, CommandDispatcher};

/// Broadcasts a text component to all connected players.
pub fn broadcast_text(
    message: TextComponent,
    query: &Query<(Entity, &StreamWriter)>,
    state: &GlobalStateResource,
) {
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

pub fn broadcast_chat_messages(
    events: Res<IncomingChatMessagePacketReceiver>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
    dispatcher: Res<CommandDispatcher>,
) {
    for (packet, sender) in events.0.try_iter() {
        if packet.message.starts_with('/') {
            let line = packet.message.trim_start_matches('/');
            let ctx = CommandContext {
                sender,
                query: &query,
                state: state.as_ref(),
            };
            dispatcher.dispatch(line, ctx);
        } else {
            let message = TextComponent::from(packet.message.clone());
            broadcast_text(message, &query, state.as_ref());
        }
    }
}
