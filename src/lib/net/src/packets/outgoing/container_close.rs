use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(packet_id = "container_close", state = "play")]
pub struct ContainerClosePacket {
    pub container_id: u8,
}
