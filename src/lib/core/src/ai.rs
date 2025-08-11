use bevy_ecs::prelude::Component;
use typename::TypeName;

use ferrumc_macros::get_registry_entry;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(TypeName, Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityKind {
    Cow,
    Zombie,
    Skeleton,
}

const COW_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cow");
const ZOMBIE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie");
const SKELETON_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:skeleton");

impl EntityKind {
    pub fn network_id(self) -> i32 {
        match self {
            EntityKind::Cow => COW_ID as i32,
            EntityKind::Zombie => ZOMBIE_ID as i32,
            EntityKind::Skeleton => SKELETON_ID as i32,
        }
    }
}

#[derive(TypeName, Component, Debug, Clone)]
pub struct Mob {
    pub kind: EntityKind,
}

#[derive(TypeName, Component, Debug, Default)]
pub struct PendingSpawn;

#[derive(TypeName, Component, Debug, Clone, Eq, PartialEq)]
pub enum AIGoal {
    Idle,
    Wander,
}

impl Default for AIGoal {
    fn default() -> Self {
        AIGoal::Idle
    }
}

/// Basic interface for AI goals.
pub trait Goal {
    /// Returns `true` when the goal has been achieved.
    fn is_complete(&self, pos: GridPos) -> bool;
}

/// A simple goal that moves an entity to a target position.
#[derive(Debug, Clone, Copy)]
pub struct MoveToGoal {
    pub target: GridPos,
}

impl Goal for MoveToGoal {
    fn is_complete(&self, pos: GridPos) -> bool {
        pos == self.target
    }
}

/// Trait representing a pathfinding strategy.
pub trait Pathfinder {
    type Node: Eq + Hash + Copy;

    fn find_path(
        &self,
        start: Self::Node,
        goal: Self::Node,
        is_walkable: impl Fn(Self::Node) -> bool,
    ) -> Option<Vec<Self::Node>>;
}

/// 2D grid position used by the default pathfinder.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    fn neighbors(self) -> [GridPos; 4] {
        [
            GridPos {
                x: self.x + 1,
                y: self.y,
            },
            GridPos {
                x: self.x - 1,
                y: self.y,
            },
            GridPos {
                x: self.x,
                y: self.y + 1,
            },
            GridPos {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    fn manhattan(self, other: GridPos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: GridPos,
    score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior.
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* pathfinder operating on a 2D grid.
pub struct AStarPathfinder;

impl Pathfinder for AStarPathfinder {
    type Node = GridPos;

    fn find_path(
        &self,
        start: GridPos,
        goal: GridPos,
        is_walkable: impl Fn(GridPos) -> bool,
    ) -> Option<Vec<GridPos>> {
        let mut open = BinaryHeap::new();
        open.push(Node {
            pos: start,
            score: 0,
        });

        let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
        let mut g_score: HashMap<GridPos, i32> = HashMap::new();
        g_score.insert(start, 0);

        while let Some(Node { pos, .. }) = open.pop() {
            if pos == goal {
                let mut path = vec![pos];
                let mut current = pos;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }

            let current_g = *g_score.get(&pos).unwrap_or(&i32::MAX);
            for neighbor in pos.neighbors() {
                if !is_walkable(neighbor) {
                    continue;
                }
                let tentative_g = current_g + 1;
                if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, pos);
                    g_score.insert(neighbor, tentative_g);
                    let f_score = tentative_g + neighbor.manhattan(goal);
                    open.push(Node {
                        pos: neighbor,
                        score: f_score,
                    });
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astar_finds_simple_path() {
        let pf = AStarPathfinder;
        let obstacles = [GridPos { x: 1, y: 0 }];
        let path = pf
            .find_path(GridPos { x: 0, y: 0 }, GridPos { x: 2, y: 0 }, |p| {
                !obstacles.contains(&p)
            })
            .expect("Path should be found");
        assert_eq!(path.first(), Some(&GridPos { x: 0, y: 0 }));
        assert_eq!(path.last(), Some(&GridPos { x: 2, y: 0 }));
        assert!(path.len() > 2); // Must route around the obstacle
    }
}
