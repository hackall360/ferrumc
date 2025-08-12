use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "entity_effect", state = "play")]
pub struct EntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub amplifier: u8,
    pub duration: VarInt,
    pub flags: u8,
}

impl EntityEffectPacket {
    pub fn new(entity_id: i32, effect_id: i32, amplifier: u8, duration: i32, flags: u8) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            effect_id: VarInt::new(effect_id),
            amplifier,
            duration: VarInt::new(duration),
            flags,
        }
    }
}
