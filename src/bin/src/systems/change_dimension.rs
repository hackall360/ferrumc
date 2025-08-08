use bevy_ecs::prelude::{EventReader, Query, Res};
use ferrumc_core::conn::change_dimension_event::ChangeDimensionEvent;
use ferrumc_net::{connection::StreamWriter, packets::outgoing::respawn::Respawn};
use ferrumc_state::GlobalStateResource;
use tracing::error;

pub fn handle_change_dimension(
    mut events: EventReader<ChangeDimensionEvent>,
    mut query: Query<&mut StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if let Ok(mut conn) = query.get_mut(event.player) {
            let packet = Respawn::new(
                format!("minecraft:{}", event.dimension),
                format!("minecraft:{}", event.dimension),
            );
            if let Err(e) = conn.send_packet_ref(&packet) {
                error!("Failed to send respawn packet: {:?}", e);
                continue;
            }
            state
                .0
                .players
                .set_dimension(event.player, event.dimension.clone());
        }
    }
}
