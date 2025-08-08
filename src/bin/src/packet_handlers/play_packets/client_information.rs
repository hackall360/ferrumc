use bevy_ecs::prelude::Res;
use ferrumc_net::ClientInformationPacketReceiver;
use tracing::info;

pub fn handle(events: Res<ClientInformationPacketReceiver>) {
    for (_, entity) in events.0.try_iter() {
        info!("Received client information from {:?}", entity);
    }
}
