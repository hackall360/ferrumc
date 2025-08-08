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
pub fn broadcast_text<'a, I>(
    message: TextComponent,
    targets: I,
    state: &GlobalStateResource,
)
where
    I: IntoIterator<Item = (Entity, &'a StreamWriter)>,
{
    let outgoing = OutgoingChatMessagePacket::new(message);
    for (entity, conn) in targets.into_iter() {
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
    mut query: Query<(
        Entity,
        &StreamWriter,
        &mut ferrumc_core::transform::position::Position,
        &mut ferrumc_core::inventory::Inventory,
        &ferrumc_core::identity::player_identity::PlayerIdentity,
    )>,
    state: Res<GlobalStateResource>,
    dispatcher: Res<CommandDispatcher>,
) {
    for (packet, sender) in events.0.try_iter() {
        if packet.message.starts_with('/') {
            let line = packet.message.trim_start_matches('/');
            let ctx = CommandContext {
                sender,
                query: &mut query,
                state: state.as_ref(),
            };
            dispatcher.dispatch(line, ctx);
        } else {
            let message = TextComponent::from(packet.message.clone());
            broadcast_text(
                message,
                query.iter_mut().map(|(e, conn, _, _, _)| (e, conn)),
                state.as_ref(),
            );
        }
    }
}
