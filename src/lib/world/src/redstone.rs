use crate::block_id::BlockId;
use crate::errors::WorldError;
use crate::tick::{BlockPos, ScheduledTick, TickManager};
use crate::World;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct PowerLevelCache {
    pub levels: HashMap<(i32, i32, i32, String), u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RedstoneComponent {
    Wire,
    Torch,
    Repeater { delay: u8, facing: Direction },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Direction> {
        match s {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }

    fn offset(self) -> (i32, i32, i32) {
        match self {
            Direction::North => (0, 0, -1),
            Direction::South => (0, 0, 1),
            Direction::East => (1, 0, 0),
            Direction::West => (-1, 0, 0),
        }
    }
}

pub fn identify_component(block: &BlockId) -> Option<RedstoneComponent> {
    block.to_block_data().and_then(|data| {
        match data.name.as_str() {
            "minecraft:redstone_wire" => Some(RedstoneComponent::Wire),
            "minecraft:redstone_torch" | "minecraft:redstone_wall_torch" => {
                Some(RedstoneComponent::Torch)
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
                Some(RedstoneComponent::Repeater { delay, facing })
            }
            _ => None,
        }
    })
}

pub fn propagate_from(world: &World, cache: &mut PowerLevelCache, start: BlockPos, power: u8) {
    let mut queue = VecDeque::new();
    queue.push_back((start, power));
    while let Some((pos, level)) = queue.pop_front() {
        let key = (pos.x, pos.y, pos.z, pos.dimension.clone());
        let current = cache.levels.get(&key).copied().unwrap_or(0);
        if level <= current {
            continue;
        }
        cache.levels.insert(key, level);
        if level == 0 {
            continue;
        }
        if level > 1 {
            let dirs = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];
            for (dx, dy, dz) in dirs.into_iter() {
                let nx = pos.x + dx;
                let ny = pos.y + dy;
                let nz = pos.z + dz;
                if let Ok(block) = world.get_block_and_fetch(nx, ny, nz, &pos.dimension) {
                    if identify_component(&block) == Some(RedstoneComponent::Wire) {
                        let next_pos = BlockPos { x: nx, y: ny, z: nz, dimension: pos.dimension.clone() };
                        queue.push_back((next_pos, level - 1));
                    }
                }
            }
        }
    }
}

pub fn tick_torch(
    world: &World,
    tm: &mut TickManager,
    pos: &BlockPos,
    block: BlockId,
) -> Result<(), WorldError> {
    {
        let mut cache = world.redstone_cache.lock().unwrap();
        propagate_from(world, &mut cache, pos.clone(), 15);
    }
    tm.schedule(ScheduledTick { pos: pos.clone(), block, delay: 1 });
    Ok(())
}

pub fn tick_repeater(
    world: &World,
    tm: &mut TickManager,
    pos: &BlockPos,
    delay: u8,
    facing: Direction,
    block: BlockId,
) -> Result<(), WorldError> {
    let input_offset = match facing {
        Direction::North => (0, 0, 1),
        Direction::South => (0, 0, -1),
        Direction::East => (-1, 0, 0),
        Direction::West => (1, 0, 0),
    };
    let output_offset = facing.offset();

    let input_key = (
        pos.x + input_offset.0,
        pos.y + input_offset.1,
        pos.z + input_offset.2,
        pos.dimension.clone(),
    );
    let input_power = {
        let cache = world.redstone_cache.lock().unwrap();
        *cache.levels.get(&input_key).unwrap_or(&0)
    };

    let output_pos = BlockPos {
        x: pos.x + output_offset.0,
        y: pos.y + output_offset.1,
        z: pos.z + output_offset.2,
        dimension: pos.dimension.clone(),
    };
    {
        let mut cache = world.redstone_cache.lock().unwrap();
        if input_power > 0 {
            propagate_from(world, &mut cache, output_pos.clone(), 15);
        } else {
            cache
                .levels
                .insert((output_pos.x, output_pos.y, output_pos.z, output_pos.dimension.clone()), 0);
        }
    }
    tm.schedule(ScheduledTick { pos: pos.clone(), block, delay: delay as u32 });
    Ok(())
}
