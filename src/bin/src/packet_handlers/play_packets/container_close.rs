use bevy_ecs::prelude::Res;
use ferrumc_net::ServerboundContainerCloseReceiver;
use tracing::trace;

pub fn handle(events: Res<ServerboundContainerCloseReceiver>) {
    for (_event, _eid) in events.0.try_iter() {
        trace!("Container closed");
    }
}

