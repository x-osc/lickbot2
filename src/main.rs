use std::sync::Arc;
use std::thread;
use std::time::Duration;

use azalea::ecs::prelude::*;
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::debug::PathfinderDebugParticles;
use azalea::pathfinder::execute::simulation::SimulationPathfinderExecutionPlugin;
use azalea::pathfinder::goals::{Goal, RadiusGoal};
use azalea::swarm::prelude::*;
use azalea::{ClientInformation, EntityRef, prelude::*};
use clap::Parser;
use parking_lot::Mutex;
use tracing::{error, warn};

use crate::commands::{CmdCtx, execute};
use crate::pvp::pvp_tick;

mod commands;
mod pvp;

#[tokio::main]
async fn main() -> AppExit {
    thread::spawn(deadlock_detection_thread);

    let args = Args::parse();
    let join_address = args.address.clone();

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_plugins(SimulationPathfinderExecutionPlugin);

    for username_or_email in &args.accounts {
        let account = if username_or_email.contains('@') {
            Account::microsoft(username_or_email).await.unwrap()
        } else {
            Account::offline(username_or_email)
        };

        builder = builder.add_account_with_state(account, State::new());
    }

    builder
        .join_delay(Duration::from_millis(100))
        .set_swarm_state(SwarmState { args })
        .start(join_address)
        .await
}

fn deadlock_detection_thread() {
    loop {
        thread::sleep(Duration::from_secs(10));
        let deadlocks = parking_lot::deadlock::check_deadlock();
        if deadlocks.is_empty() {
            continue;
        }

        error!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            error!("Deadlock #{i}");
            for t in threads {
                error!("Thread Id {:#?}", t.thread_id());
                error!("{:#?}", t.backtrace());
            }
        }
    }
}

#[derive(Clone, Component, Default)]
pub struct State {
    pub following_entity: Arc<Mutex<Option<EntityRef>>>,
    pub pvp_target: Arc<Mutex<Option<EntityRef>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            following_entity: Default::default(),
            pvp_target: Default::default(),
        }
    }
}

#[derive(Clone, Default, Resource)]
struct SwarmState {
    pub args: Args,
}

async fn handle(bot: Client, event: azalea::Event, state: State) {
    let swarm_state = bot.resource::<SwarmState>();

    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            });
            if swarm_state.args.pathfinder_debug_particles {
                bot.ecs
                    .write()
                    .entity_mut(bot.entity)
                    .insert(PathfinderDebugParticles);
            }
        }
        azalea::Event::Chat(chat) => {
            let (Some(username), content) = chat.split_sender_and_content() else {
                return;
            };

            if let Some(owner) = &swarm_state.args.owner
                && username != *owner
            {
                return;
            }

            let Some(command) = content.strip_prefix('!') else {
                return;
            };

            execute(
                command,
                CmdCtx {
                    bot: &bot,
                    state: &state,
                    chat: chat.clone(),
                },
            )
            .await;
        }
        azalea::Event::Tick => {
            follow_tick(&bot, &state);
            pvp_tick(&bot, &state);
        }
        _ => {}
    }
}

fn follow_tick(bot: &Client, state: &State) {
    // TODO: turn following into plugin
    #[allow(clippy::collapsible_if)]
    if bot.ticks_connected().is_multiple_of(5) {
        if let Some(following_entity) = state.following_entity.lock().as_ref()
            && following_entity.is_alive()
        {
            let goal = RadiusGoal::new(following_entity.position(), 3.);
            if (!bot.is_calculating_path() && !goal.success(bot.position().into()))
                || bot.is_executing_path()
            {
                bot.start_goto_with_opts(goal, PathfinderOpts::new().retry_on_no_path(false));
            } else {
                following_entity.look_at();
            }
        }
    }
}

async fn swarm_handle(swarm: Swarm, event: SwarmEvent, state: SwarmState) {
    match &event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            warn!("bot got kicked! {}", account.username());
        }
        SwarmEvent::Chat(chat) => {
            if chat.message().to_string() == "The particle was not visible for anybody"
                || chat
                    .message()
                    .to_string()
                    .contains("Displaying particle minecraft:dust")
            {
                return;
            }
            println!("{}", chat.message().to_ansi());
        }
        _ => {}
    }
}

#[derive(Parser, Debug, Clone, Default)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 1.., default_values = ["lickbot"])]
    /// Usernames or emails of the accounts to use, space separated. If it contains an '@', it will be treated as a Microsoft account, otherwise it will be treated as an offline account.
    accounts: Vec<String>,

    #[arg(short = 'A', long, default_value = "localhost:25565")]
    /// The address of the server to connect to.
    address: String,

    #[arg(short = 'o', long)]
    /// The username of the owner of the bot. If specified, the bot will only respond to commands from this user.
    owner: Option<String>,

    #[arg(short = 'P', long, default_value_t = false)]
    /// Whether to show where the bot is pathfinding to by spamming the /particle command. Off by default.
    pathfinder_debug_particles: bool,
}
