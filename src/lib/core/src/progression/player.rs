use std::collections::{HashMap, HashSet};

use super::{packets::CraftedRecipePacket, AdvancementsPacket, ProgressionRegistry};

/// Tracks advancement progress for a single player.
#[derive(Debug, Default)]
pub struct PlayerProgression {
    completed: HashMap<String, HashSet<String>>,
}

impl PlayerProgression {
    /// Marks a criterion as complete for the given advancement.
    /// When all criteria are met, packets to notify the client are returned.
    pub fn complete_criterion(
        &mut self,
        advancement_id: &str,
        criterion: &str,
        registry: &ProgressionRegistry,
    ) -> (Vec<AdvancementsPacket>, Vec<CraftedRecipePacket>) {
        let criteria = registry
            .advancements
            .get(advancement_id)
            .map(|a| a.criteria())
            .unwrap_or_default();
        if !criteria.iter().any(|c| c == criterion) {
            return (Vec::new(), Vec::new());
        }

        let entry = self
            .completed
            .entry(advancement_id.to_string())
            .or_insert_with(HashSet::new);
        entry.insert(criterion.to_string());

        if entry.len() == criteria.len() {
            let (adv_packets, recipe_packets) = self.trigger_rewards(advancement_id, registry);
            self.completed.remove(advancement_id);
            return (adv_packets, recipe_packets);
        }
        (Vec::new(), Vec::new())
    }

    fn trigger_rewards(
        &self,
        advancement_id: &str,
        registry: &ProgressionRegistry,
    ) -> (Vec<AdvancementsPacket>, Vec<CraftedRecipePacket>) {
        let adv_packet = AdvancementsPacket::grant_single(advancement_id);

        let mut recipe_packets = Vec::new();
        if let Some(adv) = registry.advancements.get(advancement_id) {
            for recipe in adv.recipe_rewards() {
                recipe_packets.push(CraftedRecipePacket::new(recipe));
            }
        }
        (vec![adv_packet], recipe_packets)
    }
}
