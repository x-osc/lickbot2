use std::thread;
use std::time::Duration;

use azalea::ecs::prelude::*;
use azalea::pathfinder::debug::PathfinderDebugParticles;
use azalea::pathfinder::execute::simulation::SimulationPathfinderExecutionPlugin;
use azalea::swarm::prelude::*;
use azalea::{ClientInformation, prelude::*};

#[tokio::main]
async fn main() -> AppExit {
    thread::spawn(deadlock_detection_thread);

    let join_address = "localhost:25565";
    let accounts = vec!["lickbot".to_string()];

    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_plugins(SimulationPathfinderExecutionPlugin);

    for username in accounts {
        let account = Account::offline(&username);
        builder = builder.add_account_with_state(account, State::new());
    }

    builder
        .join_delay(Duration::from_millis(100))
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

        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{i}");
            for t in threads {
                println!("Thread Id {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    }
}

#[derive(Clone, Component, Default)]
struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Default, Resource)]
struct SwarmState {}

async fn handle(bot: Client, event: azalea::Event, state: State) {
    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            });
            if true {
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
    match event {
        _ => {}
    }
}
