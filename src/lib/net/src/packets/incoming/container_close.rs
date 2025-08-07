use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "container_close", state = "play")]
pub struct ServerboundContainerClose {
    pub container_id: u8,
}
