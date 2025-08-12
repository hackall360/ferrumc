use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::inventory::Inventory;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use ferrumc_storage::player_data::{save_player_data, InventoryData, PlayerData, PlayerStatsData, PositionData};
use ferrumc_text::TextComponent;
use tracing::{info, trace, warn};

pub fn connection_killer(
    query: Query<(Entity, &StreamWriter, &PlayerIdentity, &Position, &Inventory)>,
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
) {
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        for (entity, conn, player_identity, position, inventory) in query.iter() {
            if disconnecting_entity == entity {
                info!(
                    "Player {} ({}) disconnected: {}",
                    player_identity.username,
                    player_identity.uuid,
                    reason.as_deref().unwrap_or("No reason")
                );
                if conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                    trace!(
                        "Sending disconnect packet to player {}",
                        player_identity.username
                    );
                    if let Err(e) = conn.send_packet_ref(
                        &ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
                            reason: TextComponent::from(
                                reason.as_deref().unwrap_or("Disconnected"),
                            ),
                        },
                    ) {
                        warn!(
                            "Failed to send disconnect packet to player {}: {:?}",
                            player_identity.username, e
                        );
                    }
                } else {
                    trace!(
                        "Connection for player {} is not running, skipping disconnect packet",
                        player_identity.username
                    );
                }

                let pdata = PlayerData {
                    inventory: InventoryData::from(inventory),
                    position: PositionData {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                    },
                    stats: PlayerStatsData::default(),
                    advancements: Vec::new(),
                };
                let _ = save_player_data(
                    state.0.world.backend(),
                    player_identity.uuid.as_u128(),
                    &pdata,
                );
                cmd.entity(entity).despawn();
            } else {
                // Broadcast the disconnection to other players
            }
        }
    }
}
