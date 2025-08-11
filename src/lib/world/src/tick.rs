use crate::block_id::BlockId;
use crate::errors::WorldError;
use crate::redstone::{self, Direction};
use crate::World;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// Position of a block in the world.
#[derive(Clone, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dimension: String,
}

/// A scheduled tick for a block.
#[derive(Clone, Debug)]
pub struct ScheduledTick {
    pub pos: BlockPos,
    pub block: BlockId,
    pub delay: u32,
}

#[derive(Default)]
pub struct TickManager {
    pub scheduled: HashMap<(i32, i32, String), VecDeque<ScheduledTick>>,
    pub random: HashMap<(i32, i32, String), Vec<BlockPos>>,
}

impl TickManager {
    pub fn schedule(&mut self, tick: ScheduledTick) {
        let key = (tick.pos.x >> 4, tick.pos.z >> 4, tick.pos.dimension.clone());
        self.scheduled.entry(key).or_default().push_back(tick);
    }

    pub fn schedule_random(&mut self, pos: BlockPos) {
        let key = (pos.x >> 4, pos.z >> 4, pos.dimension.clone());
        self.random.entry(key).or_default().push(pos);
    }

    pub fn tick_world(&mut self, world: &World) -> Result<(), WorldError> {
        // Handle scheduled ticks
        let mut to_process = Vec::new();
        for queue in self.scheduled.values_mut() {
            let mut i = 0;
            while i < queue.len() {
                if queue[i].delay == 0 {
                    let tick = queue.remove(i).unwrap();
                    to_process.push(tick);
                } else {
                    queue[i].delay -= 1;
                    i += 1;
                }
            }
        }
        self.scheduled.retain(|_, v| !v.is_empty());
        for tick in to_process {
            tick_block(world, self, &tick.pos, tick.block)?;
        }

        // Handle random ticks – process all positions deterministically
        let positions: Vec<BlockPos> = self
            .random
            .values()
            .flat_map(|v| v.clone())
            .collect();
        for pos in positions {
            let block_id = world.get_block_and_fetch(pos.x, pos.y, pos.z, &pos.dimension)?;
            tick_block(world, self, &pos, block_id)?;
        }
        Ok(())
    }

    /// Remove all scheduled and random ticks for the given chunk.
    pub fn cleanup_chunk(&mut self, chunk_x: i32, chunk_z: i32, dimension: &str) {
        let key = (chunk_x, chunk_z, dimension.to_string());
        self.scheduled.remove(&key);
        self.random.remove(&key);
    }

    /// Remove all ticks associated with a dimension.
    pub fn cleanup_dimension(&mut self, dimension: &str) {
        self.scheduled.retain(|(_, _, dim), _| dim != dimension);
        self.random.retain(|(_, _, dim), _| dim != dimension);
    }
}

fn tick_block(world: &World, tm: &mut TickManager, pos: &BlockPos, block: BlockId) -> Result<(), WorldError> {
    if let Some(data) = block.to_block_data() {
        match data.name.as_str() {
            "minecraft:water" => water_tick(world, tm, pos)?,
            "minecraft:wheat" => crop_tick(world, pos, &data)?,
            "minecraft:redstone_torch" | "minecraft:redstone_wall_torch" => {
                redstone::tick_torch(world, tm, pos, block)?
            }
            "minecraft:repeater" => {
                let delay = data
                    .properties
                    .as_ref()
                    .and_then(|p| p.get("delay"))
                    .and_then(|v| v.parse::<u8>().ok())
                    .unwrap_or(1);
                let facing = data
                    .properties
                    .as_ref()
                    .and_then(|p| p.get("facing"))
                    .and_then(|v| Direction::from_str(v))
                    .unwrap_or(Direction::North);
                redstone::tick_repeater(world, tm, pos, delay, facing, block)?
            }
            _ => {}
        }
    }
    Ok(())
}

fn water_tick(world: &World, tm: &mut TickManager, pos: &BlockPos) -> Result<(), WorldError> {
    let dirs = [(0, -1, 0), (1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];
    for (dx, dy, dz) in dirs.iter() {
        let nx = pos.x + dx;
        let ny = pos.y + dy;
        let nz = pos.z + dz;
        if world.get_block_and_fetch(nx, ny, nz, &pos.dimension)? == BlockId::default() {
            let mut props = BTreeMap::new();
            props.insert("level".to_string(), "0".to_string());
            let water = crate::vanilla_chunk_format::BlockData {
                name: "minecraft:water".to_string(),
                properties: Some(props),
            };
            world.set_block_and_fetch(nx, ny, nz, &pos.dimension, water.clone())?;
            tm.schedule(ScheduledTick {
                pos: BlockPos { x: nx, y: ny, z: nz, dimension: pos.dimension.clone() },
                block: water.to_block_id(),
                delay: 1,
            });
        }
    }
    Ok(())
}

fn crop_tick(world: &World, pos: &BlockPos, data: &crate::vanilla_chunk_format::BlockData) -> Result<(), WorldError> {
    let mut props = data.properties.clone().unwrap_or_default();
    let age = props
        .get("age")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);
    if age < 7 {
        props.insert("age".to_string(), (age + 1).to_string());
        let new_data = crate::vanilla_chunk_format::BlockData {
            name: data.name.clone(),
            properties: Some(props),
        };
        world.set_block_and_fetch(pos.x, pos.y, pos.z, &pos.dimension, new_data)?;
    }
    Ok(())
}

/// Helper used by tests or other modules to schedule a tick for a block
pub fn schedule_block_tick(world: &World, x: i32, y: i32, z: i32, dimension: &str, delay: u32) {
    let block = world
        .get_block_and_fetch(x, y, z, dimension)
        .unwrap_or_default();
    let pos = BlockPos { x, y, z, dimension: dimension.to_string() };
    let mut guard = world.tick_manager.lock().unwrap();
    guard.schedule(ScheduledTick { pos, block, delay });
}

/// Helper to register a position for random ticks.
pub fn schedule_random_tick(world: &World, x: i32, y: i32, z: i32, dimension: &str) {
    let pos = BlockPos { x, y, z, dimension: dimension.to_string() };
    let mut guard = world.tick_manager.lock().unwrap();
    guard.schedule_random(pos);
}
