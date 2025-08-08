use bevy_ecs::entity::Entity;
use crossbeam_queue::SegQueue;
use dashmap::DashMap;
use ferrumc_world::block_id::BlockId;

#[derive(Debug, Default)]
pub struct PlayerList {
    pub player_list: DashMap<Entity, (u128, String)>,
    pub held_items: DashMap<Entity, [BlockId; 2]>,
    pub dimensions: DashMap<Entity, String>,
    pub disconnection_queue: SegQueue<(Entity, Option<String>)>,
}

impl PlayerList {
    pub fn is_connected(&self, entity: Entity) -> bool {
        self.player_list.contains_key(&entity)
    }

    pub fn disconnect(&self, entity: Entity, reason: Option<String>) {
        self.player_list.remove(&entity);
        self.held_items.remove(&entity);
        self.dimensions.remove(&entity);
        self.disconnection_queue.push((entity, reason));
    }

    pub fn set_held_item(&self, entity: Entity, hand: usize, block: BlockId) {
        self.held_items
            .entry(entity)
            .or_insert([BlockId::default(); 2])[hand] = block;
    }

    pub fn get_held_item(&self, entity: Entity, hand: usize) -> Option<BlockId> {
        self.held_items.get(&entity).map(|v| v[hand])
    }

    pub fn set_dimension(&self, entity: Entity, dimension: String) {
        self.dimensions.insert(entity, dimension);
    }

    pub fn get_dimension(&self, entity: Entity) -> Option<String> {
        self.dimensions.get(&entity).map(|v| v.clone())
    }
}
