#![allow(unsafe_code)]

pub mod errors;

use crate::errors::PluginError;
use bevy_ecs::prelude::Resource;
use libloading::{Library, Symbol};
use std::path::Path;
use std::sync::Arc;
use tracing::warn;

/// Trait implemented by all dynamically loaded plugins.
pub trait Plugin: Send + Sync {
    /// Name of the plugin for logging purposes.
    fn name(&self) -> &'static str;

    /// Called whenever a chat message is received.
    fn on_chat_message(&self, _msg: &mut String) {}

    /// Called after a block edit has occurred.
    fn on_block_edit(&self, _pos: (i32, i32, i32), _block: u32) {}

    /// Called when an entity is spawned into the world.
    fn on_entity_spawn(&self, _entity: u64) {}

    /// Called before a command is dispatched.
    fn on_command(&self, _command: &str) {}
}

type PluginCreate = unsafe fn() -> Box<dyn Plugin>;

/// Manages loading plugins and dispatching events to them.
#[derive(Default, Resource)]
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,    // Loaded plugin instances
    _libs: Vec<Arc<Library>>,         // Keep libraries alive
}

impl PluginManager {
    /// Loads all `.so` files from a directory as plugins.
    pub fn load_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<(), PluginError> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("so") {
                continue;
            }
            unsafe {
                let lib = Library::new(&path).map_err(|e| PluginError::Load(e.to_string()))?;
                let ctor: Symbol<PluginCreate> =
                    lib.get(b"create_plugin").map_err(|e| PluginError::Load(e.to_string()))?;
                let plugin = ctor();
                self.plugins.push(plugin);
                self._libs.push(Arc::new(lib));
            }
        }
        Ok(())
    }

    fn with_plugins<F>(&self, mut f: F)
    where
        F: FnMut(&dyn Plugin),
    {
        for plugin in &self.plugins {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(plugin.as_ref());
            }));
            if result.is_err() {
                warn!(plugin = plugin.name(), "plugin panicked while handling event");
            }
        }
    }

    /// Dispatches a chat message event to all plugins.
    pub fn on_chat_message(&self, msg: &mut String) {
        self.with_plugins(|p| p.on_chat_message(msg));
    }

    /// Dispatches a block edit event to all plugins.
    pub fn on_block_edit(&self, pos: (i32, i32, i32), block: u32) {
        self.with_plugins(|p| p.on_block_edit(pos, block));
    }

    /// Dispatches an entity spawn event to all plugins.
    pub fn on_entity_spawn(&self, entity: u64) {
        self.with_plugins(|p| p.on_entity_spawn(entity));
    }

    /// Dispatches a command invocation event to all plugins.
    pub fn on_command(&self, command: &str) {
        self.with_plugins(|p| p.on_command(command));
    }
}
