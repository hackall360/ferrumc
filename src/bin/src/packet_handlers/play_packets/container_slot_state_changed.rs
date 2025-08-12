use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::inventory::{Inventory, ItemStack};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::container_set_content::ContainerSetContentPacket;
use ferrumc_net::packets::outgoing::container_set_slot::ContainerSetSlotPacket;
use ferrumc_net::packets::outgoing::crafted_recipe::CraftedRecipePacket;
use ferrumc_net::ContainerSlotStateChangedReceiver;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_id::BlockId;
use tracing::debug;

pub fn handle(
    events: Res<ContainerSlotStateChangedReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<(Entity, &StreamWriter, &mut Inventory)>,
) {
    for (event, eid) in events.0.try_iter() {
        let Ok((entity, conn, mut inv)) = query.get_mut(eid) else {
            debug!("Could not get inventory for entity {:?}", eid);
            continue;
        };
        if !state.0.players.is_connected(entity) {
            continue;
        }
        let slot_index = event.slot as usize;
        let new_stack = match &event.item.item {
            PrefixedOptional::Some(data) => {
                let block_id = BlockId::from_varint(data.item_id);
                let nbt = if data.nbt.is_empty() {
                    None
                } else {
                    Some(data.nbt.clone())
                };
                Some(ItemStack::new(block_id, data.count, 64, nbt))
            }
            PrefixedOptional::None => None,
        };
        inv.set_slot(slot_index, new_stack);
        let packet = ContainerSetSlotPacket::new(
            0,
            0,
            event.slot,
            inv.get_slot(slot_index).and_then(|s| s.as_ref()),
        );
        if let Err(e) = conn.send_packet_ref(&packet) {
            debug!("Failed to send container slot update: {:?}", e);
        }
        let content_packet = ContainerSetContentPacket::from_inventory(&inv);
        if let Err(e) = conn.send_packet_ref(&content_packet) {
            debug!("Failed to send container content update: {:?}", e);
        }
        if slot_index == 0 {
            let recipe_packet = CraftedRecipePacket::new("ferrumc:test");
            if let Err(e) = conn.send_packet_ref(&recipe_packet) {
                debug!("Failed to send crafted recipe: {:?}", e);
            }
        }
    }
}

