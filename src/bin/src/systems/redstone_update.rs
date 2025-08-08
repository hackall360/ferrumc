use bevy_ecs::prelude::ResMut;
use std::collections::VecDeque;

#[derive(Default)]
pub struct RedstoneScheduler {
    pub queue: VecDeque<RedstoneUpdate>,
}

#[derive(Clone)]
pub struct RedstoneUpdate {
    pub position: (i32, i32, i32),
    pub delay: u8,
}

pub fn run_redstone_updates(mut scheduler: ResMut<RedstoneScheduler>) {
    let mut i = 0;
    while i < scheduler.queue.len() {
        if scheduler.queue[i].delay == 0 {
            scheduler.queue.remove(i);
        } else {
            scheduler.queue[i].delay -= 1;
            i += 1;
        }
    }
}
