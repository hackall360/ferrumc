use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "update_health", state = "play")]
pub struct UpdateHealthPacket {
    pub health: f32,
    pub food: VarInt,
    pub saturation: f32,
}

impl UpdateHealthPacket {
    pub fn new(health: f32, food: i32, saturation: f32) -> Self {
        Self {
            health,
            food: VarInt::new(food),
            saturation,
        }
    }
}
