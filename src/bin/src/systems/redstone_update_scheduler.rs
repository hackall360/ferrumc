use bevy_ecs::prelude::{Entity, Query, Res, ResMut, Resource};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::{network_position::NetworkPosition, var_int::VarInt};
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use std::collections::VecDeque;
use tracing::error;

#[derive(Clone)]
pub struct RedstoneUpdate {
    pub position: NetworkPosition,
    pub block: BlockId,
    pub delay: u32,
}

#[derive(Resource, Default)]
pub struct RedstoneUpdateQueue {
    pub queue: VecDeque<RedstoneUpdate>,
}

impl RedstoneUpdateQueue {
    pub fn schedule(&mut self, update: RedstoneUpdate) {
        self.queue.push_back(update);
    }
}

pub fn tick(
    mut queue: ResMut<RedstoneUpdateQueue>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    let mut remaining = VecDeque::new();
    while let Some(mut update) = queue.queue.pop_front() {
        if update.delay > 0 {
            update.delay -= 1;
            remaining.push_back(update);
        } else {
            let packet = BlockUpdate {
                location: update.position,
                block_id: VarInt::from(update.block),
            };
            for (entity, conn) in query.iter() {
                if !state.0.players.is_connected(entity) {
                    continue;
                }
                if let Err(e) = conn.send_packet_ref(&packet) {
                    error!("Failed to send block update: {:?}", e);
                }
            }
        }
    }
    queue.queue = remaining;
}
