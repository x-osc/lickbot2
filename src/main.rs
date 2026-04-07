use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

use azalea::ecs::prelude::*;
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::debug::PathfinderDebugParticles;
use azalea::pathfinder::execute::simulation::SimulationPathfinderExecutionPlugin;
use azalea::pathfinder::goals::{Goal, RadiusGoal};
use azalea::swarm::prelude::*;
use azalea::{ClientInformation, EntityRef, prelude::*, protocol};
use azalea_viaversion::ViaVersionPlugin;
use clap::Parser;
use parking_lot::Mutex;
use shadow_rs::shadow;
use tracing::{debug, error, info, trace, warn};

use crate::commands::{CmdCtx, execute};
use crate::pvp::pvp_tick;

mod commands;
mod item;
mod pvp;

shadow!(build);

#[tokio::main]
async fn main() -> AppExit {
    thread::spawn(deadlock_detection_thread);

    let args = Args::parse();

    let filter = if (args.verbose) != 0 {
        get_rust_log(args.verbose as i8).to_owned()
    } else {
        std::env::var("RUST_LOG")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| get_rust_log(0).to_owned())
    };

    let fmt_subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(fmt_subscriber).unwrap();

    info!("starting {}", short_version());
    debug!("{}", single_line_version());

    let join_address = args.server.clone();

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle);

    if !args.disable_simulation_patfinder {
        debug!("simulation pathfinder is enabled");
        builder = builder.add_plugins(SimulationPathfinderExecutionPlugin);
    }

    if args.mc_version != protocol::packets::VERSION_NAME {
        info!("starting viaproxy for version {}...", args.mc_version);
        builder = builder.add_plugins(ViaVersionPlugin::start(&args.mc_version).await);
    }

    info!(
        "joining {} with accounts [ {} ]",
        args.server,
        args.accounts
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ")
    );

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
                debug!("pathfinder_debug_particles are enabled");
                bot.ecs
                    .write()
                    .entity_mut(bot.entity)
                    .insert(PathfinderDebugParticles);
            }
        }
        azalea::Event::Spawn => {
            info!("{} has logged in", bot.username())
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
                    chat: chat,
                },
            )
            .await;
        }
        azalea::Event::Tick => {
            follow_tick(&bot, &state);
            pvp_tick(&bot, &state);
        }
        azalea::Event::Death(death) => {
            // TODO: why dis happen + fix lib
            let Some(death) = death else {
                return;
            };
            info!("{} has died!", bot.username());
            debug!("reason: {:?}", death);
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
                trace!("setting new follow goal: {:?}", goal);
                bot.start_goto_with_opts(goal, PathfinderOpts::new().retry_on_no_path(false));
            } else {
                following_entity.look_at();
            }
        }
    }
}

async fn swarm_handle(_swarm: Swarm, event: SwarmEvent, _state: SwarmState) {
    match &event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            warn!("{} got disconnected!", account.username());
        }
        SwarmEvent::Chat(chat) => {
            if chat
                .message()
                .to_string()
                .contains("The particle was not visible for anybody")
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

fn short_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        format!(
            "{name} v{version}",
            name = build::PROJECT_NAME,
            version = build::PKG_VERSION
        )
    })
}

fn single_line_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        format!(
            "{name} v{version} ({git}{dirty} on {branch} @ {time} with {rust})",
            name = build::PROJECT_NAME,
            version = build::PKG_VERSION,
            git = build::SHORT_COMMIT,
            dirty = if build::GIT_CLEAN { "" } else { "+" },
            branch = build::BRANCH,
            time = build::BUILD_TIME,
            rust = build::RUST_VERSION
        )
    })
}

fn version_clap() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        format!(
            "v{version}\n\n{git}{dirty} on {branch},\ncompiled @ {time}\nwith {rust}",
            version = build::PKG_VERSION,
            git = build::SHORT_COMMIT,
            dirty = if build::GIT_CLEAN { "" } else { "+" },
            branch = build::BRANCH,
            time = build::BUILD_TIME,
            rust = build::RUST_VERSION
        )
    })
}

fn get_rust_log(verbosity: i8) -> &'static str {
    match verbosity {
        ..=-2 => "error,lickbot2=error",
        -1 => "warn,lickbot2=warn",
        0 => "warn,lickbot2=info",
        1 => "warn,lickbot2=debug",
        2 => "info,lickbot2=debug",
        3 => "debug,lickbot2=trace",
        4.. => "trace,lickbot2=trace",
    }
}

#[derive(Parser, Debug, Clone, Default)]
#[command(version = version_clap(), about, long_about = None, next_line_help = true, max_term_width = 150)]
struct Args {
    #[arg(short = 'a', long, num_args = 1.., required = true)]
    /// Usernames or emails of the accounts to use, space separated. If it is an email, it will be treated as a Microsoft account, otherwise treated as an offline account.
    accounts: Vec<String>,

    #[arg(short = 's', long, required = true)]
    /// The address of the server to connect to.
    server: String,

    #[arg(short = 'o', long)]
    /// The username of the owner of the bot. If specified, the bot will only respond to commands from this user.
    owner: Option<String>,

    #[arg(short = 'm', long, default_value = protocol::packets::VERSION_NAME)]
    /// Target minecraft version
    mc_version: String,

    #[arg(short = 'S', long)]
    /// Disable the pathfinder's postprocessor which smooths the path to make it more realistic.
    disable_simulation_patfinder: bool,

    #[arg(short = 'P', long)]
    /// Show where the bot is pathfinding to by spamming the /particle command.
    pathfinder_debug_particles: bool,

    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    /// Increase logging verbosity
    verbose: u8,
}
