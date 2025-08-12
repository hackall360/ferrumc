use std::collections::HashMap;
use std::io::Write;

use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_text::TextComponent;

#[derive(NetEncode, Clone)]
pub struct ObjectiveInfo {
    pub display_name: TextComponent,
    pub render_type: i8,
}

#[derive(NetEncode, Clone)]
#[packet(packet_id = "set_objective", state = "play")]
pub struct UpdateObjectivesPacket {
    pub objective_name: String,
    pub mode: i8,
    pub info: Option<ObjectiveInfo>,
}

impl UpdateObjectivesPacket {
    pub fn create(name: &str, info: ObjectiveInfo) -> Self {
        Self { objective_name: name.to_string(), mode: 0, info: Some(info) }
    }

    pub fn remove(name: &str) -> Self {
        Self { objective_name: name.to_string(), mode: 1, info: None }
    }

    pub fn update(name: &str, info: ObjectiveInfo) -> Self {
        Self { objective_name: name.to_string(), mode: 2, info: Some(info) }
    }
}

#[derive(NetEncode, Clone)]
#[packet(packet_id = "set_score", state = "play")]
pub struct UpdateScorePacket {
    pub entity_name: String,
    pub action: i8,
    pub objective_name: String,
    pub value: Option<VarInt>,
}

impl UpdateScorePacket {
    pub fn set(entity_name: String, objective_name: String, value: VarInt) -> Self {
        Self { entity_name, action: 0, objective_name, value: Some(value) }
    }

    pub fn remove(entity_name: String, objective_name: String) -> Self {
        Self { entity_name, action: 1, objective_name, value: None }
    }
}

#[derive(Clone)]
pub enum ObjectiveRenderType {
    Integer,
    Hearts,
}

#[derive(Clone)]
pub struct Objective {
    pub name: String,
    pub display_name: TextComponent,
    pub render_type: ObjectiveRenderType,
}

pub struct Scoreboard {
    pub objectives: HashMap<String, Objective>,
    pub scores: HashMap<(String, String), i32>, // (entity, objective)
}

impl Scoreboard {
    pub fn new() -> Self {
        Self { objectives: HashMap::new(), scores: HashMap::new() }
    }

    pub fn add_objective(
        &mut self,
        name: String,
        display_name: TextComponent,
        render_type: ObjectiveRenderType,
    ) -> UpdateObjectivesPacket {
        let info = ObjectiveInfo {
            display_name: display_name.clone(),
            render_type: match render_type {
                ObjectiveRenderType::Integer => 0,
                ObjectiveRenderType::Hearts => 1,
            },
        };
        let packet = UpdateObjectivesPacket::create(&name, info.clone());
        self.objectives.insert(
            name.clone(),
            Objective { name, display_name, render_type },
        );
        packet
    }

    pub fn remove_objective(&mut self, name: &str) -> Option<UpdateObjectivesPacket> {
        if self.objectives.remove(name).is_some() {
            Some(UpdateObjectivesPacket::remove(name))
        } else {
            None
        }
    }

    pub fn set_score(
        &mut self,
        entity: String,
        objective: String,
        value: i32,
    ) -> UpdateScorePacket {
        self.scores.insert((entity.clone(), objective.clone()), value);
        UpdateScorePacket::set(entity, objective, VarInt::new(value))
    }

    pub fn remove_score(&mut self, entity: &str, objective: &str) -> Option<UpdateScorePacket> {
        if self.scores.remove(&(entity.to_string(), objective.to_string())).is_some() {
            Some(UpdateScorePacket::remove(entity.to_string(), objective.to_string()))
        } else {
            None
        }
    }
}
