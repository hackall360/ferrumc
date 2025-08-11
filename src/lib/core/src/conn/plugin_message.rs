use bevy_ecs::prelude::{Entity, Event, EventWriter};

/// Event fired when a plugin wishes to register a new plugin messaging channel
/// for a player. The network layer forwards this as a `minecraft:register`
/// custom payload packet.
#[derive(Event, Debug)]
pub struct PluginChannelRegisterEvent {
    pub entity: Entity,
    pub channel: String,
}

/// Event fired when a plugin wants to send data on a plugin channel to a
/// player.
#[derive(Event, Debug)]
pub struct PluginMessageSendEvent {
    pub entity: Entity,
    pub channel: String,
    pub data: Vec<u8>,
}

/// Registers a plugin messaging channel for the given player.
pub fn register_plugin_channel(
    mut writer: EventWriter<PluginChannelRegisterEvent>,
    entity: Entity,
    channel: impl Into<String>,
) {
    writer.write(PluginChannelRegisterEvent {
        entity,
        channel: channel.into(),
    });
}

/// Sends raw plugin channel data to the given player.
pub fn send_plugin_message(
    mut writer: EventWriter<PluginMessageSendEvent>,
    entity: Entity,
    channel: impl Into<String>,
    data: Vec<u8>,
) {
    writer.write(PluginMessageSendEvent {
        entity,
        channel: channel.into(),
        data,
    });
}
