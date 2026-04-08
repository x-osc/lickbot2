#![allow(dead_code)]

use azalea::inventory::{ItemStack, Menu};

pub trait LickbotMenuExt {
    /// Returns the contents of the player's inventory.
    fn player_slots(&self) -> Vec<ItemStack>;

    /// Returns the contents of the player's inventory, not including the hotbar.
    fn player_slots_without_hotbar(&self) -> Vec<ItemStack>;

    /// Returns the contents of the player's hotbar.
    fn hotbar_slots(&self) -> Vec<ItemStack>;

    /// Converts a hotbar-relative index (0-8) into the corresponding absolute slot index for the menu.
    ///
    /// # Panics
    /// This will panic if `i` is out of bounds for the hotbar. Make sure to only submit values in the range 0-8.
    fn hotbar_index_to_slot_index(&self, i: usize) -> usize;

    /// Converts an absolute slot index into a hotbar-relative index (0-8).
    ///
    /// Returns `Some(index)` if the slot is within the hotbar,
    /// otherwise returns `None`
    fn slot_index_to_hotbar_index(&self, i: usize) -> Option<usize>;
}

impl LickbotMenuExt for Menu {
    fn player_slots(&self) -> Vec<ItemStack> {
        self.slots()[self.player_slots_range()].to_vec()
    }

    fn player_slots_without_hotbar(&self) -> Vec<ItemStack> {
        self.slots()[self.player_slots_without_hotbar_range()].to_vec()
    }

    fn hotbar_slots(&self) -> Vec<ItemStack> {
        self.slots()[self.hotbar_slots_range()].to_vec()
    }

    fn hotbar_index_to_slot_index(&self, i: usize) -> usize {
        assert!(i <= 8, "hotbar index out of bounds");

        self.hotbar_slots_range().start() + i
    }

    fn slot_index_to_hotbar_index(&self, i: usize) -> Option<usize> {
        if self.hotbar_slots_range().contains(&i) {
            Some(i - self.hotbar_slots_range().start())
        } else {
            None
        }
    }
}
