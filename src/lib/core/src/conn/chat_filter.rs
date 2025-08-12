use bevy_ecs::entity::Entity;
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Trait for filtering chat messages from clients.
/// Return `None` to drop the message, or `Some` to continue with a possibly
/// modified version.
pub trait ChatFilter: Send + Sync + 'static {
    fn filter(&self, entity: Entity, message: &str) -> Option<String>;
}

static CHAT_FILTERS: Lazy<RwLock<Vec<Box<dyn ChatFilter>>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Register a new chat filter. Filters are executed in the order they are
/// registered.
pub fn register_filter<F: ChatFilter>(filter: F) {
    if let Ok(mut filters) = CHAT_FILTERS.write() {
        filters.push(Box::new(filter));
    }
}

/// Apply all registered chat filters to a message.
/// If any filter returns `None`, the chain is aborted and `None` is returned.
pub fn filter_message(entity: Entity, message: &str) -> Option<String> {
    let mut current = Some(message.to_string());
    if let Ok(filters) = CHAT_FILTERS.read() {
        for f in filters.iter() {
            if let Some(ref msg) = current {
                current = f.filter(entity, msg);
            } else {
                break;
            }
        }
    }
    current
}
