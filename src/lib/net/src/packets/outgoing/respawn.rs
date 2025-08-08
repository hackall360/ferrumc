use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "respawn", state = "play")]
pub struct Respawn {
    pub dimension_type: String,
    pub dimension_name: String,
}

impl Respawn {
    pub fn new(dimension_type: String, dimension_name: String) -> Self {
        Self {
            dimension_type,
            dimension_name,
        }
    }
}
