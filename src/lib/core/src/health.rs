use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
}
