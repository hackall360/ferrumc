use bevy_ecs::prelude::Res;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// System that advances the world by one tick.
pub fn tick_world(state: Res<GlobalStateResource>) {
    if let Err(e) = state.0.world.tick() {
        error!("World tick failed: {e}");
    }
}
