use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use std::io::Write;

pub use ferrumc_net_codec::net_types::global_pos::GlobalPos;

#[derive(NetEncode)]
#[packet(packet_id = "login", state = "play")]
pub struct LoginPlayPacket<'a> {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_length: VarInt,
    pub dimension_names: &'a [&'a str],
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: &'a str,
    pub seed_hash: i64,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: PrefixedOptional<GlobalPos<'a>>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

impl<'a> LoginPlayPacket<'a> {
    pub fn new(conn_id: i32, dimension_id: i32, death: Option<GlobalPos<'a>>) -> Self {
        let dimension_name = match dimension_id {
            -1 => "minecraft:the_nether",
            1 => "minecraft:the_end",
            _ => "minecraft:overworld",
        };
        Self {
            entity_id: conn_id,
            is_hardcore: false,
            dimension_length: VarInt::from(3),
            dimension_names: &["minecraft:overworld", "minecraft:the_nether", "minecraft:the_end"],
            max_players: VarInt::from(get_global_config().max_players as i32),
            view_distance: VarInt::from(get_global_config().chunk_render_distance as i32),
            simulation_distance: VarInt::from(get_global_config().chunk_render_distance as i32),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::new(dimension_id),
            dimension_name,
            seed_hash: 0,
            gamemode: 1,
            previous_gamemode: -1,
            is_debug: false,
            is_flat: false,
            death_location: PrefixedOptional::new(death),
            portal_cooldown: VarInt::from(0),
            sea_level: VarInt::from(63),
            enforces_secure_chat: false,
        }
    }
}
