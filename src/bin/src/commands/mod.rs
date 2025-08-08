use std::convert::Infallible;
use std::marker::PhantomData;

use brigadier_rs::{
    float_64, integer_i32, literal, BuildExecute, CommandArgument, CommandParser, Then,
};
use bevy_ecs::prelude::{Entity, Query, Resource};
use ferrumc_core::{
    identity::player_identity::PlayerIdentity,
    inventory::{Inventory, ItemStack},
    transform::position::Position,
};
use ferrumc_net::{
    connection::StreamWriter,
    packets::outgoing::{
        container_set_slot::ContainerSetSlotPacket,
        player_info_update::{PlayerInfoUpdatePacket, PlayerWithActions},
        synchronize_player_position::SynchronizePlayerPositionPacket,
    },
};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use ferrumc_world::{block_id::BlockId, vanilla_chunk_format::BlockData};
use nom::IResult;
use tracing::warn;

use crate::systems::chat_message;

/// Context provided to command handlers when they are executed.
#[derive(Copy, Clone)]
pub struct CommandContext<'a> {
    /// Entity that issued the command.
    pub sender: Entity,
    /// Pointer to the player query.
    pub query: *mut Query<
        'a,
        'a,
        (
            Entity,
            &'a StreamWriter,
            &'a mut Position,
            &'a mut Inventory,
            &'a PlayerIdentity,
        ),
    >,
    /// Global server state.
    pub state: *const GlobalStateResource,
    /// Lifetime marker.
    _marker: PhantomData<&'a ()>,
}

/// Dispatcher that routes parsed commands to their handlers.
#[derive(Default, Resource)]
pub struct CommandDispatcher {
    commands:
        Vec<Box<dyn for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync + 'static>>,
}

impl CommandDispatcher {
    /// Creates a new empty dispatcher.
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    /// Registers a command parser.
    pub fn register(&mut self, parser: impl for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync + 'static) {
        self.commands.push(Box::new(parser));
    }

    /// Dispatches a command line to the appropriate parser.
    pub fn dispatch(&self, line: &str, ctx: CommandContext) {
        use nom::Finish;
        for cmd in &self.commands {
            if cmd.execute(ctx, line).finish().is_ok() {
                return;
            }
        }
        warn!("Unknown command: {}", line);
    }
}

/// Argument parser that consumes the next space-delimited word.
pub struct WordArgument;

impl<S> CommandArgument<S, String> for WordArgument {
    fn parse<'a>(&self, _source: S, input: &'a str) -> IResult<&'a str, String, brigadier_rs::CommandError<'a>> {
        let input = input.trim_start();
        let mut end = 0;
        for (idx, ch) in input.char_indices() {
            if ch.is_whitespace() {
                break;
            }
            end = idx + ch.len_utf8();
        }
        let (word, rest) = input.split_at(end);
        Ok((rest, word.to_string()))
    }
}

impl<S, E> Then<E> for WordArgument {
    type Output = brigadier_rs::parsers::CommandThen<Self, E, String, S>;

    fn then(self, executor: E) -> Self::Output {
        brigadier_rs::parsers::CommandThen { argument: self, executor }
    }
}

pub fn word() -> WordArgument {
    WordArgument
}

/// Argument parser that consumes the remainder of the line.
pub struct RestArgument;

impl<S> CommandArgument<S, String> for RestArgument {
    fn parse<'a>(&self, _source: S, input: &'a str) -> IResult<&'a str, String, brigadier_rs::CommandError<'a>> {
        let input = input.trim_start();
        Ok(("", input.to_string()))
    }
}

impl<S, E> Then<E> for RestArgument {
    type Output = brigadier_rs::parsers::CommandThen<Self, E, String, S>;

    fn then(self, executor: E) -> Self::Output {
        brigadier_rs::parsers::CommandThen { argument: self, executor }
    }
}

pub fn rest() -> RestArgument {
    RestArgument
}

/// `/say` command that broadcasts a message to all players.
pub fn say_command() -> impl for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync {
    literal("say")
        .then(rest().build_exec(|ctx, msg: String| {
            unsafe {
                let query = &mut *ctx.query;
                let state = &*ctx.state;
                let text = TextComponent::from(msg);
                chat_message::broadcast_text(
                    text,
                    query.iter_mut().map(|(e, conn, _, _, _)| (e, conn)),
                    state,
                );
            }
            Ok::<(), Infallible>(())
        }))
        .build_exec(|_ctx| Ok::<(), Infallible>(()))
}

/// `/tp <x> <y> <z>` command.
pub fn tp_command() -> impl for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync {
    literal("tp")
        .then(
            float_64("x").then(
                float_64("y").then(
                    float_64("z").build_exec(|ctx, x, y, z| {
                        unsafe {
                            let query = &mut *ctx.query;
                            if let Ok((_, conn, mut pos, _, _)) = query.get_mut(ctx.sender) {
                                pos.x = x;
                                pos.y = y;
                                pos.z = z;
                                let teleport_id_i32 = (rand::random::<u32>() & 0x3FFF_FFFF) as i32;
                                let packet = SynchronizePlayerPositionPacket::new(
                                    (x, y, z),
                                    (0.0, 0.0, 0.0),
                                    0.0,
                                    0.0,
                                    0,
                                    VarInt::new(teleport_id_i32),
                                );
                                let _ = conn.send_packet_ref(&packet);
                            } else {
                                warn!("Sender entity {:?} not found for tp", ctx.sender);
                            }
                        }
                        Ok::<(), Infallible>(())
                    }),
                ),
            ),
        )
        .build_exec(|_ctx| Ok::<(), Infallible>(()))
}

/// `/give <item> [count]` command.
pub fn give_command() -> impl for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync {
    literal("give")
        .then(
            word().then(integer_i32("count").build_exec(|ctx, item: String, count: i32| {
                give_handler(ctx, item, count as u8)
            }))
            .build_exec(|ctx, item: String| give_handler(ctx, item, 1)),
        )
        .build_exec(|_ctx| Ok::<(), Infallible>(()))
}

fn give_handler(ctx: CommandContext, item: String, count: u8) -> Result<(), Infallible> {
    unsafe {
        let query = &mut *ctx.query;
        if let Ok((_, conn, _, mut inv, _)) = query.get_mut(ctx.sender) {
            let name = if item.contains(':') {
                item
            } else {
                format!("minecraft:{}", item)
            };
            let block = BlockId::from_block_data(&BlockData { name, properties: None });
            let stack = ItemStack::new(block, count);
            inv.hotbar[0] = Some(stack.clone());
            let packet = ContainerSetSlotPacket::new(0, 0, 0, Some(&stack));
            let _ = conn.send_packet_ref(&packet);
        } else {
            warn!("Sender entity {:?} not found for give", ctx.sender);
        }
    }
    Ok(())
}

/// `/gamemode <mode>` command.
pub fn gamemode_command() -> impl for<'a> CommandParser<CommandContext<'a>, ()> + Send + Sync {
    literal("gamemode")
        .then(word().build_exec(|ctx, mode: String| {
            unsafe {
                let query = &mut *ctx.query;
                let state = &*ctx.state;
                if let Ok((entity, _conn, _pos, _inv, identity)) = query.get_mut(ctx.sender) {
                    let gm = match mode.as_str() {
                        "0" | "survival" => 0,
                        "1" | "creative" => 1,
                        "2" | "adventure" => 2,
                        "3" | "spectator" => 3,
                        _ => {
                            warn!("Unknown gamemode: {}", mode);
                            return Ok::<(), Infallible>(());
                        }
                    };
                    // drop borrow before iterating over all players
                    let uuid = identity.short_uuid;
                    drop((entity, _conn, _pos, _inv, identity));

                    let packet =
                        PlayerInfoUpdatePacket::with_players(vec![PlayerWithActions::update_game_mode(
                            uuid, gm,
                        )]);

                    for (e, conn, _, _, _) in query.iter_mut() {
                        if !state.0.players.is_connected(e) {
                            continue;
                        }
                        let _ = conn.send_packet_ref(&packet);
                    }
                } else {
                    warn!("Sender entity {:?} not found for gamemode", ctx.sender);
                }
            }
            Ok::<(), Infallible>(())
        }))
        .build_exec(|_ctx| Ok::<(), Infallible>(()))
}

