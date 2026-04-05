use azalea::prelude::*;
use tracing::trace;

use crate::State;

pub fn pvp_tick(bot: &Client, state: &State) {
    let target_guard = state.pvp_target.lock();
    let Some(target) = target_guard.as_ref() else {
        return;
    };

    if !target.is_alive() {
        return;
    }

    if bot.has_attack_cooldown() {
        return;
    }

    trace!("Attacking entity {}", target.id());

    let target_pos = target.position();
    if bot.eye_position().distance_to(target_pos) < 4. {
        target.look_at();
    }

    if bot.eye_position().distance_to(target_pos) < 3. {
        target.attack();
    }
}
