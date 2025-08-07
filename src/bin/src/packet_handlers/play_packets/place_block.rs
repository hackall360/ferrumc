use std::sync::Arc;

use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlaceBlockReceiver;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use ferrumc_world::vanilla_chunk_format::BlockData;
use tracing::{debug, trace};

pub fn handle(
    events: Res<PlaceBlockReceiver>,
    state: Res<GlobalStateResource>,
    conn_q: Query<(Entity, &StreamWriter)>,
    pos_q: Query<(&Position, &CollisionBounds)>,
) {
    'ev_loop: for (event, eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            let Ok((entity, conn)) = conn_q.get(eid) else {
                debug!("Could not get connection for entity {:?}", eid);
                continue;
            };
            if !state.0.players.is_connected(entity) {
                trace!("Entity {:?} is not connected", entity);
                continue;
            }
            let hand = event.hand.0 as usize;
            if hand > 1 {
                debug!("Invalid hand");
                continue;
            }
            let Some(mut block_id) = state.0.players.get_held_item(entity, hand) else {
                trace!("Player has no block in hand {}", hand);
                continue;
            };
            if block_id == BlockId::default() {
                trace!("Player attempted to place air");
                continue;
            }

            let mut block_data = block_id.to_block_data().unwrap_or(BlockData::default());
            if let Some(props) = block_data.properties.as_mut() {
                if props.contains_key("facing") {
                    let facing = match event.face.0 {
                        0 => "down",
                        1 => "up",
                        2 => "north",
                        3 => "south",
                        4 => "west",
                        5 => "east",
                        _ => "north",
                    };
                    props.insert("facing".into(), facing.into());
                } else if props.contains_key("axis") {
                    let axis = match event.face.0 {
                        0 | 1 => "y",
                        2 | 3 => "z",
                        4 | 5 => "x",
                        _ => "y",
                    };
                    props.insert("axis".into(), axis.into());
                }
            }
            block_id = BlockId::from_block_data(&block_data);

            let mut chunk = match state.0.world.load_chunk_owned(
                event.position.x >> 4,
                event.position.z >> 4,
                "overworld",
            ) {
                Ok(chunk) => chunk,
                Err(e) => {
                    debug!("Failed to load chunk: {:?}", e);
                    continue 'ev_loop;
                }
            };
            let block_clicked =
                chunk.get_block(event.position.x, event.position.y as i32, event.position.z)?;
            trace!("Block clicked: {:?}", block_clicked);
            let (x_block_offset, y_block_offset, z_block_offset) = match event.face.0 {
                0 => (0, -1, 0),
                1 => (0, 1, 0),
                2 => (0, 0, -1),
                3 => (0, 0, 1),
                4 => (-1, 0, 0),
                5 => (1, 0, 0),
                _ => (0, 0, 0),
            };
            let (x, y, z) = (
                event.position.x + x_block_offset,
                event.position.y + y_block_offset,
                event.position.z + z_block_offset,
            );
            let does_collide = {
                pos_q.into_iter().any(|(pos, bounds)| {
                    bounds.collides(
                        (pos.x, pos.y, pos.z),
                        &CollisionBounds {
                            x_offset_start: 0.0,
                            x_offset_end: 1.0,
                            y_offset_start: 0.0,
                            y_offset_end: 1.0,
                            z_offset_start: 0.0,
                            z_offset_end: 1.0,
                        },
                        (x as f64, y as f64, z as f64),
                    )
                })
            };
            if does_collide {
                trace!("Block placement collided with entity");
                continue 'ev_loop;
            }

            chunk.set_block(x & 0xF, y as i32, z & 0xF, block_id)?;

            let chunk_packet = BlockUpdate {
                location: NetworkPosition { x, y, z },
                block_id: VarInt::from(block_id),
            };
            conn.send_packet_ref(&chunk_packet)?;
            let ack_packet = BlockChangeAck {
                sequence: event.sequence,
            };
            conn.send_packet_ref(&ack_packet)?;

            state.0.world.save_chunk(Arc::new(chunk))?;
        };
        if let Err(e) = &res {
            debug!("Failed to handle place block: {:?}", e);
        }
    }
}
