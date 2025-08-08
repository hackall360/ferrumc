use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "client_information", state = "play")]
pub struct ClientInformationPacket;
