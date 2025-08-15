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
    Comparator { facing: Direction },
    Observer { facing: Direction },
    Piston { facing: Direction },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Direction> {
        match s {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None,
        }
    }

    fn offset(self) -> (i32, i32, i32) {
        match self {
            Direction::North => (0, 0, -1),
            Direction::South => (0, 0, 1),
            Direction::East => (1, 0, 0),
            Direction::West => (-1, 0, 0),
            Direction::Up => (0, 1, 0),
            Direction::Down => (0, -1, 0),
        }
    }
}

pub fn identify_component(block: &BlockId) -> Option<RedstoneComponent> {
    block
        .to_block_data()
        .and_then(|data| match data.name.as_str() {
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
            "minecraft:comparator" => {
                let facing = data
                    .properties
                    .as_ref()
                    .and_then(|p| p.get("facing"))
                    .and_then(|v| Direction::from_str(v))
                    .unwrap_or(Direction::North);
                Some(RedstoneComponent::Comparator { facing })
            }
            "minecraft:observer" => {
                let facing = data
                    .properties
                    .as_ref()
                    .and_then(|p| p.get("facing"))
                    .and_then(|v| Direction::from_str(v))
                    .unwrap_or(Direction::North);
                Some(RedstoneComponent::Observer { facing })
            }
            "minecraft:piston" | "minecraft:sticky_piston" => {
                let facing = data
                    .properties
                    .as_ref()
                    .and_then(|p| p.get("facing"))
                    .and_then(|v| Direction::from_str(v))
                    .unwrap_or(Direction::North);
                Some(RedstoneComponent::Piston { facing })
            }
            _ => None,
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
                        let next_pos = BlockPos {
                            x: nx,
                            y: ny,
                            z: nz,
                            dimension: pos.dimension.clone(),
                        };
                        queue.push_back((next_pos, level - 1));
                    }
                }
            }
        }
    }
}

fn component_delay(comp: &RedstoneComponent) -> u32 {
    match comp {
        RedstoneComponent::Wire => 0,
        RedstoneComponent::Torch => 1,
        RedstoneComponent::Repeater { delay, .. } => *delay as u32,
        RedstoneComponent::Comparator { .. } => 1,
        RedstoneComponent::Observer { .. } => 1,
        RedstoneComponent::Piston { .. } => 1,
    }
}

fn schedule_component_update(world: &World, tm: &mut TickManager, pos: BlockPos) {
    if let Ok(block) = world.get_block_and_fetch(pos.x, pos.y, pos.z, &pos.dimension) {
        if let Some(comp) = identify_component(&block) {
            let delay = component_delay(&comp);
            if delay == 0 {
                let mut cache = world.redstone_cache.lock().unwrap();
                propagate_from(world, &mut cache, pos, 15);
            } else {
                tm.schedule(ScheduledTick { pos, block, delay });
            }
        }
    }
}

pub fn propagate_block_update(world: &World, tm: &mut TickManager, pos: &BlockPos) {
    let neighbors = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
    ];
    for (dx, dy, dz) in neighbors {
        let npos = BlockPos {
            x: pos.x + dx,
            y: pos.y + dy,
            z: pos.z + dz,
            dimension: pos.dimension.clone(),
        };
        schedule_component_update(world, tm, npos);
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
    propagate_block_update(world, tm, pos);
    tm.schedule(ScheduledTick {
        pos: pos.clone(),
        block,
        delay: 1,
    });
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
            cache.levels.insert(
                (
                    output_pos.x,
                    output_pos.y,
                    output_pos.z,
                    output_pos.dimension.clone(),
                ),
                0,
            );
        }
    }
    schedule_component_update(world, tm, output_pos);
    propagate_block_update(world, tm, pos);
    tm.schedule(ScheduledTick {
        pos: pos.clone(),
        block,
        delay: delay as u32,
    });
    Ok(())
}

pub fn tick_comparator(
    world: &World,
    tm: &mut TickManager,
    pos: &BlockPos,
    facing: Direction,
    block: BlockId,
) -> Result<(), WorldError> {
    let input_offset = match facing {
        Direction::North => (0, 0, 1),
        Direction::South => (0, 0, -1),
        Direction::East => (-1, 0, 0),
        Direction::West => (1, 0, 0),
        Direction::Up => (0, -1, 0),
        Direction::Down => (0, 1, 0),
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
            propagate_from(world, &mut cache, output_pos.clone(), input_power);
        } else {
            cache.levels.insert(
                (
                    output_pos.x,
                    output_pos.y,
                    output_pos.z,
                    output_pos.dimension.clone(),
                ),
                0,
            );
        }
    }
    schedule_component_update(world, tm, output_pos);
    propagate_block_update(world, tm, pos);
    tm.schedule(ScheduledTick {
        pos: pos.clone(),
        block,
        delay: 1,
    });
    Ok(())
}

pub fn tick_observer(
    world: &World,
    tm: &mut TickManager,
    pos: &BlockPos,
    facing: Direction,
    block: BlockId,
) -> Result<(), WorldError> {
    let output_offset = facing.offset();
    let output_pos = BlockPos {
        x: pos.x + output_offset.0,
        y: pos.y + output_offset.1,
        z: pos.z + output_offset.2,
        dimension: pos.dimension.clone(),
    };
    let current_power = {
        let cache = world.redstone_cache.lock().unwrap();
        *cache
            .levels
            .get(&(
                output_pos.x,
                output_pos.y,
                output_pos.z,
                output_pos.dimension.clone(),
            ))
            .unwrap_or(&0)
    };
    {
        let mut cache = world.redstone_cache.lock().unwrap();
        if current_power == 0 {
            propagate_from(world, &mut cache, output_pos.clone(), 15);
        } else {
            cache.levels.insert(
                (
                    output_pos.x,
                    output_pos.y,
                    output_pos.z,
                    output_pos.dimension.clone(),
                ),
                0,
            );
        }
    }
    schedule_component_update(world, tm, output_pos);
    propagate_block_update(world, tm, pos);
    tm.schedule(ScheduledTick {
        pos: pos.clone(),
        block,
        delay: 1,
    });
    Ok(())
}

pub fn tick_piston(
    world: &World,
    tm: &mut TickManager,
    pos: &BlockPos,
    facing: Direction,
    block: BlockId,
) -> Result<(), WorldError> {
    let front = facing.offset();
    let front_pos = BlockPos {
        x: pos.x + front.0,
        y: pos.y + front.1,
        z: pos.z + front.2,
        dimension: pos.dimension.clone(),
    };
    schedule_component_update(world, tm, front_pos);
    propagate_block_update(world, tm, pos);
    tm.schedule(ScheduledTick {
        pos: pos.clone(),
        block,
        delay: 1,
    });
    Ok(())
}
