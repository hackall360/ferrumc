use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "entity_position_sync", state = "play")]
pub struct TeleportEntityPacket {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl TeleportEntityPacket {
    pub fn new(short_id: i32, position: &Position, angle: &Rotation, on_ground: bool) -> Self {
        // Todo: Add velocity parameters if needed
        Self {
            entity_id: VarInt::new(short_id),
            x: position.x,
            y: position.y,
            z: position.z,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
            yaw: angle.yaw,
            pitch: angle.pitch,
            on_ground,
        }
    }
}
