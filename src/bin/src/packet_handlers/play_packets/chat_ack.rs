use bevy_ecs::prelude::Res;
use ferrumc_net::ChatAckPacketReceiver;
use tracing::info;

pub fn handle(events: Res<ChatAckPacketReceiver>) {
    for (_, entity) in events.0.try_iter() {
        info!("Received chat acknowledgment from {:?}", entity);
    }
}
