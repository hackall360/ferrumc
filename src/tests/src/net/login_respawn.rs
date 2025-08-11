use ferrumc_net::packets::outgoing::login_play::{GlobalPos, LoginPlayPacket};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;

#[test]
fn login_packet_has_no_death_location() {
    let packet = LoginPlayPacket::new(1, "minecraft:overworld", None);
    assert!(packet.death_location.is_none());
    assert_eq!(packet.dimension_name, "minecraft:overworld");
}

#[test]
fn respawn_packet_contains_death_location() {
    let death = GlobalPos::new("minecraft:overworld", NetworkPosition { x: 1, y: 2, z: 3 });
    let packet = LoginPlayPacket::new(1, "minecraft:overworld", Some(death));
    assert!(packet.death_location.is_some());
}
