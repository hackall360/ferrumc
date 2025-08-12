use bevy_ecs::prelude::Component;
use typename::TypeName;

/// Identity information for a player, including permission level.
///
/// # Examples
/// ```
/// use ferrumc_core::identity::player_identity::PlayerIdentity;
/// let id = PlayerIdentity::new("Steve".into(), 42);
/// assert_eq!(id.permission_level, 0);
/// ```
#[derive(TypeName, Debug, Component, Default, Clone)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: uuid::Uuid,
    pub short_uuid: i32,
    /// Permission level of the player.
    pub permission_level: u8,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username,
            uuid: uuid::Uuid::from_u128(uuid),
            short_uuid: uuid as i32,
            permission_level: 0,
        }
    }
}
