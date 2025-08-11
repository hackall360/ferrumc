use crate::errors::NetError;
use crate::packets::packet_events::PluginMessageEvent;
use crate::{CustomPayloadPacketReceiver, connection::StreamWriter};
use bevy_ecs::prelude::{EventReader, EventWriter, Query, Res};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::conn::plugin_message::{PluginChannelRegisterEvent, PluginMessageSendEvent};
use tokio::net::TcpListener;
use tracing::{debug, error, warn};

pub async fn create_server_listener() -> Result<TcpListener, NetError> {
    let config = get_global_config();
    let server_addy = format!("{}:{}", config.host, config.port);
    let server_addy = server_addy.as_str();

    let listener = match TcpListener::bind(server_addy).await {
        Ok(l) => Ok::<TcpListener, std::io::Error>(l),
        Err(e) => {
            error!("Failed to bind to addy: {}", server_addy);
            error!("Perhaps the port {} is already in use?", config.port);

            Err(e)
        }
    };

    debug!("Server listening on {}", server_addy);

    Ok(listener?)
}

/// Routes plugin channel traffic between the network layer and the ECS world.
pub fn route_plugin_messages(
    receiver: Res<CustomPayloadPacketReceiver>,
    mut incoming_writer: EventWriter<PluginMessageEvent>,
    mut register_events: EventReader<PluginChannelRegisterEvent>,
    mut send_events: EventReader<PluginMessageSendEvent>,
    mut query: Query<&StreamWriter>,
) {
    for (packet, entity) in receiver.0.try_iter() {
        incoming_writer.write(PluginMessageEvent {
            entity,
            channel: packet.channel,
            data: packet.data,
        });
    }

    for evt in register_events.read() {
        if let Ok(writer) = query.get(evt.entity) {
            let mut data = evt.channel.clone().into_bytes();
            data.push(0);
            let packet = crate::packets::outgoing::custom_payload::CustomPayloadPacket::new(
                "minecraft:register".to_string(),
                data,
            );
            if let Err(e) = writer.send_packet(packet) {
                warn!("Failed to send channel register packet: {:?}", e);
            }
        }
    }

    for evt in send_events.read() {
        if let Ok(writer) = query.get(evt.entity) {
            let packet = crate::packets::outgoing::custom_payload::CustomPayloadPacket::new(
                evt.channel.clone(),
                evt.data.clone(),
            );
            if let Err(e) = writer.send_packet(packet) {
                warn!("Failed to send plugin message: {:?}", e);
            }
        }
    }
}
