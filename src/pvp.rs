use azalea::prelude::*;
use tracing::{debug, trace};

use crate::{State, item::weapon::best_weapon_in_hotbar};

pub fn pvp_tick(bot: &Client, state: &State) {
    let target_guard = state.pvp_target.lock();
    let Some(target) = target_guard.as_ref() else {
        return;
    };

    if !target.is_alive() {
        return;
    }

    if bot.ticks_connected().is_multiple_of(20) && !bot.is_mining() {
        // TODO: fix sort_inventory (its async now)

        let best_slot = best_weapon_in_hotbar(&bot.menu());
        if best_slot as u8 != bot.selected_hotbar_slot() {
            debug!(
                "Selecting weapon {} at slot {}",
                bot.menu().slots()[bot.menu().hotbar_slots_range()][best_slot].kind(),
                best_slot
            );
            bot.set_selected_hotbar_slot(best_slot as u8);
        }
    }

    if bot.has_attack_cooldown() {
        return;
    }

    trace!("Attacking entity {}", target.id());

    if bot.eye_position().distance_to(target.position()) < 4.5
        || bot.eye_position().distance_to(target.eye_position()) < 4.5
    {
        target.look_at();
    }

    if bot.eye_position().distance_to(target.position()) < 3.
        || bot.eye_position().distance_to(target.eye_position()) < 3.
    {
        target.attack();
    }
}
