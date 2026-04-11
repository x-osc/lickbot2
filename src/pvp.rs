use std::sync::Arc;

use azalea::EntityRef;
use parking_lot::Mutex;
use tracing::{debug, trace};

use crate::{
    FollowingData, async_task,
    item::{
        inventory::{should_sort_inventory, sort_inventory},
        weapon::best_weapon_in_hotbar,
    },
    task::{Task, TaskContext, TaskStatus},
};

pub struct PvpTask {
    pub target: Arc<Mutex<EntityRef>>,
}

impl PvpTask {
    pub fn new(target: EntityRef) -> Self {
        Self {
            target: Arc::new(Mutex::new(target)),
        }
    }
}

impl Task for PvpTask {
    fn launch(&mut self, ctx: TaskContext) {
        ctx.state
            .following_data
            .lock()
            .replace(FollowingData::new(self.target.lock().clone()));
    }

    fn tick(&mut self, ctx: TaskContext) -> TaskStatus {
        let bot = ctx.bot;

        let target = self.target.lock();

        if !target.is_alive() {
            return TaskStatus::Finished;
        }

        if bot.ticks_connected().is_multiple_of(100) && !bot.is_mining() {
            if should_sort_inventory(&bot.menu()) {
                debug!("sorting inventory..");
                return TaskStatus::Push(Box::new(async_task!(|ctx| {
                    sort_inventory(&ctx.bot).await;
                    ctx.bot.wait_ticks(20).await;
                })));
            }

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
            return TaskStatus::Continue;
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

        TaskStatus::Continue
    }

    fn cancel(&mut self, _ctx: TaskContext) {}
}
