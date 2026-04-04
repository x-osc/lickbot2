use std::thread;
use std::time::Duration;

use azalea::ecs::prelude::*;
use azalea::pathfinder::debug::PathfinderDebugParticles;
use azalea::pathfinder::execute::simulation::SimulationPathfinderExecutionPlugin;
use azalea::swarm::prelude::*;
use azalea::{ClientInformation, prelude::*};
use clap::Parser;
use tracing::{error, warn};

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

#[derive(Parser, Debug, Clone, Default)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 1.., default_values = ["lickbot"])]
    /// Usernames or emails of the accounts to use, space separated. If it contains an '@', it will be treated as a Microsoft account, otherwise it will be treated as an offline account.
    accounts: Vec<String>,

    #[arg(short = 'A', long, default_value = "localhost:25565")]
    /// The address of the server to connect to.
    address: String,

    #[arg(short = 'P', long, default_value_t = false)]
    /// Whether to show where the bot is pathfinding to by spamming the /particle command. Off by default.
    pathfinder_debug_particles: bool,
}

#[derive(Clone, Component, Default)]
struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Default, Resource)]
struct SwarmState {
    pub args: Args,
}

async fn handle(bot: Client, event: azalea::Event, state: State) {
    let swarm_state = bot.resource::<SwarmState>();

    #[allow(clippy::single_match)]
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
        _ => {}
    }
}

async fn swarm_handle(swarm: Swarm, event: SwarmEvent, state: SwarmState) {
    #[allow(clippy::single_match)]
    match &event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            warn!("bot got kicked! {}", account.username());
        }
        _ => {}
    }
}
