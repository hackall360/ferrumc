use bevy_ecs::event::Events;
use bevy_ecs::system::RunSystemOnce;
use bevy_ecs::world::World;
use ferrumc_core::chunks::block_break_progress::BlockBreakProgress;
use ferrumc_core::inventory::{Inventory, ItemStack};
use ferrumc_net::packets::packet_events::{PlayerDiggingEvent, UseItemEvent};
use ferrumc_net::server::{handle_player_digging, handle_use_item};
use ferrumc_world::block_id::BlockId;

#[test]
fn player_digging_progress_tracking() {
    let mut world = World::new();
    world.insert_resource(BlockBreakProgress::default());
    world.insert_resource(Events::<PlayerDiggingEvent>::default());
    let entity = world.spawn_empty().id();

    {
        let mut events = world.resource_mut::<Events<PlayerDiggingEvent>>();
        events.send(PlayerDiggingEvent {
            entity,
            status: 0,
            position: ferrumc_core::transform::position::Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            face: 0,
        });
    }
    world.run_system_once(handle_player_digging);
    let tracker = world.resource::<BlockBreakProgress>();
    assert!(tracker.get(1, 2, 3, "overworld").is_some());
}

#[test]
#[ignore]
fn use_item_consumes_stack() {
    let mut world = World::new();
    world.insert_resource(Events::<UseItemEvent>::default());
    let entity = world
        .spawn(Inventory {
            hotbar: Default::default(),
            main: Default::default(),
            equipment: Default::default(),
            offhand: None,
        })
        .id();

    {
        let mut inv = world.get_mut::<Inventory>(entity).unwrap();
        inv.hotbar[0] = Some(ItemStack::new(BlockId(1), 1, 64, None));
    }

    {
        let mut events = world.resource_mut::<Events<UseItemEvent>>();
        events.send(UseItemEvent { entity, hand: 0 });
    }
    world.run_system_once(handle_use_item);
    let inv = world.get::<Inventory>(entity).unwrap();
    assert!(inv.hotbar[0].is_none());
}
