use bevy_ecs::prelude::{Entity, Query, Resource};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use std::collections::HashMap;
use tracing::warn;

use crate::systems::chat_message;

/// Context provided to command handlers when they are executed.
pub struct CommandContext<'a> {
    /// Entity that issued the command.
    pub sender: Entity,
    /// Query used to access all connected clients.
    pub query: &'a Query<'a, 'a, (Entity, &'a StreamWriter)>,
    /// Global server state.
    pub state: &'a GlobalStateResource,
}

/// Trait implemented by individual command handlers.
pub trait CommandHandler: Send + Sync {
    /// Handles the command with the provided arguments and context.
    fn handle(&self, args: &str, ctx: CommandContext);
}

/// Dispatcher that routes parsed commands to their handlers.
#[derive(Default, Resource)]
pub struct CommandDispatcher {
    handlers: HashMap<&'static str, Box<dyn CommandHandler>>,
}

impl CommandDispatcher {
    /// Creates a new empty dispatcher.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a command handler for a given command name.
    pub fn register(&mut self, name: &'static str, handler: impl CommandHandler + 'static) {
        self.handlers.insert(name, Box::new(handler));
    }

    /// Dispatches a command line to the appropriate handler.
    pub fn dispatch<'a>(&self, line: &str, ctx: CommandContext<'a>) {
        let mut parts = line.splitn(2, ' ');
        if let Some(cmd) = parts.next() {
            if let Some(handler) = self.handlers.get(cmd) {
                let args = parts.next().unwrap_or("");
                handler.handle(args, ctx);
            } else {
                warn!("Unknown command: {}", cmd);
            }
        }
    }
}

/// Simple `/say` command that broadcasts a message to all players.
pub struct SayCommand;

impl CommandHandler for SayCommand {
    fn handle(&self, args: &str, ctx: CommandContext) {
        let text = TextComponent::from(args.to_string());
        chat_message::broadcast_text(text, ctx.query, ctx.state);
    }
}
