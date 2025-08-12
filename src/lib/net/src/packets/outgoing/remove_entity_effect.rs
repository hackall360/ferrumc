use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "remove_entity_effect", state = "play")]
pub struct RemoveEntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
}

impl RemoveEntityEffectPacket {
    pub fn new(entity_id: i32, effect_id: i32) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            effect_id: VarInt::new(effect_id),
        }
    }
}
