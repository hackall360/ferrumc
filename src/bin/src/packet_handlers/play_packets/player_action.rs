use std::sync::Arc;

use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlayerActionReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::vanilla_chunk_format::BlockData;
use tracing::{debug, error, trace};

fn send_ack(
    query: &Query<(Entity, &StreamWriter)>,
    state: &GlobalStateResource,
    eid: Entity,
    sequence: VarInt,
) -> Result<(), BinaryError> {
    if let Ok((entity, conn)) = query.get(eid) {
        if state.0.players.is_connected(entity) {
            let ack_packet = BlockChangeAck { sequence };
            conn.send_packet_ref(&ack_packet)?;
        }
    } else {
        debug!("Failed to find connection for entity {:?}", eid);
    }
    Ok(())
}

pub fn handle(
    events: Res<PlayerActionReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &StreamWriter)>,
) {
    // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
    for (event, trigger_eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            match event.status.0 {
                0 => {
                    // Started digging
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                1 => {
                    // Cancelled digging
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                2 => {
                    // Finished digging
                    let mut chunk = match state.0.clone().world.load_chunk_owned(
                        event.location.x >> 4,
                        event.location.z >> 4,
                        "overworld",
                    ) {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            trace!("Chunk not found, generating new chunk: {:?}", e);
                            let gen = state.0.clone().terrain_generator;
                            let bx = event.location.x >> 4;
                            let bz = event.location.z >> 4;
                            let biome = gen.biome_at(bx, bz);
                            gen.generate_chunk_for_biome(bx, bz, biome)?
                        }
                    };
                    let (relative_x, relative_y, relative_z) = (
                        event.location.x.abs() % 16,
                        event.location.y as i32,
                        event.location.z.abs() % 16,
                    );
                    chunk.set_block(relative_x, relative_y, relative_z, BlockData::default())?;
                    state.0.world.save_chunk(Arc::new(chunk))?;
                    let block_update_packet = BlockUpdate {
                        location: event.location.clone(),
                        block_id: VarInt::from(BlockId::default()),
                    };
                    for (eid, conn) in query.iter() {
                        if !state.0.players.is_connected(*eid) {
                            continue;
                        }
                        conn.send_packet_ref(&block_update_packet)?;
                    }
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                3 => {
                    // Drop item stack
                    state
                        .0
                        .players
                        .set_held_item(trigger_eid, 0, BlockId::default());
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                4 => {
                    // Drop single item
                    state
                        .0
                        .players
                        .set_held_item(trigger_eid, 0, BlockId::default());
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                5 => {
                    // Release use item
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                6 => {
                    // Swap hand items
                    let main = state
                        .0
                        .players
                        .get_held_item(trigger_eid, 0)
                        .unwrap_or_default();
                    let off = state
                        .0
                        .players
                        .get_held_item(trigger_eid, 1)
                        .unwrap_or_default();
                    state.0.players.set_held_item(trigger_eid, 0, off);
                    state.0.players.set_held_item(trigger_eid, 1, main);
                    send_ack(&query, &state, trigger_eid, event.sequence)?;
                }
                _ => {}
            };
        };
        if res.is_err() {
            error!("Error handling player action: {:?}", res);
        }
    }
}
