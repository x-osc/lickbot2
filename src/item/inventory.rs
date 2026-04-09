use azalea::{
    container::ContainerHandle,
    inventory::{Menu, operations::SwapClick},
    prelude::*,
    registry::builtin::ItemKind,
};
use tracing::{debug, warn};

use crate::item::extensions::LickbotMenuExt;

// hotbar relative
const AXE_HOTBAR_INDEX: usize = 0;
const SWORD_HOTBAR_INDEX: usize = 1;
const PICKAXE_HOTBAR_INDEX: usize = 2;
const SHOVEL_HOTBAR_INDEX: usize = 3;
const HOE_HOTBAR_INDEX: usize = 4;

// slot relative
const HEAD_SLOT_INDEX: usize = 5;
const CHEST_SLOT_INDEX: usize = 6;
const LEGS_SLOT_INDEX: usize = 7;
const FEET_SLOT_INDEX: usize = 8;

pub async fn sort_inventory(bot: &Client) {
    let inventory_menu = bot.menu();
    let mut lazyinv = LazyInventory::new(bot);

    if let Some((best_slot, best_kind)) = best_axe(&inventory_menu)
        && best_slot != inventory_menu.hotbar_index_to_slot_index(AXE_HOTBAR_INDEX)
    {
        debug!("Swapping {best_kind} at slot {best_slot} to hotbar slot {AXE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            // target slot is hotbar relative this isnt a mistake lmao
            target_slot: AXE_HOTBAR_INDEX as u8,
        });
        bot.wait_ticks(1).await;
    }
    if let Some((best_slot, best_kind)) = best_sword(&inventory_menu)
        && best_slot != inventory_menu.hotbar_index_to_slot_index(SWORD_HOTBAR_INDEX)
    {
        debug!("Swapping {best_kind} at slot {best_slot} to hotbar slot {SWORD_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: SWORD_HOTBAR_INDEX as u8,
        });
        bot.wait_ticks(1).await;
    }
    if let Some((best_slot, best_kind)) = best_pickaxe(&inventory_menu)
        && best_slot != inventory_menu.hotbar_index_to_slot_index(PICKAXE_HOTBAR_INDEX)
    {
        debug!("Swapping {best_kind} at slot {best_slot} to hotbar slot {PICKAXE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: PICKAXE_HOTBAR_INDEX as u8,
        });
        bot.wait_ticks(1).await;
    }
    if let Some((best_slot, best_kind)) = best_shovel(&inventory_menu)
        && best_slot != inventory_menu.hotbar_index_to_slot_index(SHOVEL_HOTBAR_INDEX)
    {
        debug!("Swapping {best_kind} at slot {best_slot} to hotbar slot {SHOVEL_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: SHOVEL_HOTBAR_INDEX as u8,
        });
        bot.wait_ticks(1).await;
    }
    if let Some((best_slot, best_kind)) = best_hoe(&inventory_menu)
        && best_slot != inventory_menu.hotbar_index_to_slot_index(HOE_HOTBAR_INDEX)
    {
        debug!("Swapping {best_kind} at slot {best_slot} to hotbar slot {HOE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: HOE_HOTBAR_INDEX as u8,
        });
        bot.wait_ticks(1).await;
    }

    // TODO: need to make empty slot in inventory
    if let Some((best_slot, best_kind)) = best_helmet(&inventory_menu)
        && best_slot != HEAD_SLOT_INDEX
    {
        debug!("Swapping {best_kind} at slot {best_slot} to armor slot {HEAD_SLOT_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };

        if inventory_menu.slot(HEAD_SLOT_INDEX).is_some() {
            inventory_ref.shift_click(HEAD_SLOT_INDEX);
            bot.wait_ticks(1).await;
        }

        inventory_ref.shift_click(best_slot);
        bot.wait_ticks(1).await;
    };

    if let Some((best_slot, best_kind)) = best_chestplate(&inventory_menu)
        && best_slot != CHEST_SLOT_INDEX
    {
        debug!("Swapping {best_kind} at slot {best_slot} to armor slot {CHEST_SLOT_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };

        if inventory_menu.slot(CHEST_SLOT_INDEX).is_some() {
            inventory_ref.shift_click(CHEST_SLOT_INDEX);
            bot.wait_ticks(1).await;
        }

        inventory_ref.shift_click(best_slot);
        bot.wait_ticks(1).await;
    };

    if let Some((best_slot, best_kind)) = best_leggings(&inventory_menu)
        && best_slot != LEGS_SLOT_INDEX
    {
        debug!("Swapping {best_kind} at slot {best_slot} to armor slot {LEGS_SLOT_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };

        if inventory_menu.slot(LEGS_SLOT_INDEX).is_some() {
            inventory_ref.shift_click(LEGS_SLOT_INDEX);
            bot.wait_ticks(1).await;
        }

        inventory_ref.shift_click(best_slot);
        bot.wait_ticks(1).await;
    };

    if let Some((best_slot, best_kind)) = best_boots(&inventory_menu)
        && best_slot != FEET_SLOT_INDEX
    {
        debug!("Swapping {best_kind} at slot {best_slot} to armor slot {FEET_SLOT_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };

        if inventory_menu.slot(FEET_SLOT_INDEX).is_some() {
            inventory_ref.shift_click(FEET_SLOT_INDEX);
            bot.wait_ticks(1).await;
        }

        inventory_ref.shift_click(best_slot);
        bot.wait_ticks(1).await;
    };
}

/// only opens inventory if necessary
struct LazyInventory<'a> {
    bot: &'a Client,
    inventory_ref: Option<ContainerHandle>,
}

impl<'a> LazyInventory<'a> {
    fn new(bot: &'a Client) -> Self {
        Self {
            bot,
            inventory_ref: None,
        }
    }

    /// returns a reference to the currently open inventory, opening it if necessary
    ///
    /// Returns none if another container is already open.
    fn get(&mut self) -> Option<&ContainerHandle> {
        if self.inventory_ref.is_none() {
            match self.bot.open_inventory() {
                Some(r) => self.inventory_ref = Some(r),
                None => {
                    warn!("bot tried to open inventory while another container was open");
                    return None;
                }
            }
        }
        self.inventory_ref.as_ref()
    }
}

const AXE_ORDER: [ItemKind; 7] = [
    ItemKind::WoodenAxe,
    ItemKind::GoldenAxe,
    ItemKind::StoneAxe,
    ItemKind::CopperAxe,
    ItemKind::IronAxe,
    ItemKind::DiamondAxe,
    ItemKind::NetheriteAxe,
];

const SWORD_ORDER: [ItemKind; 7] = [
    ItemKind::WoodenSword,
    ItemKind::GoldenSword,
    ItemKind::StoneSword,
    ItemKind::CopperSword,
    ItemKind::IronSword,
    ItemKind::DiamondSword,
    ItemKind::NetheriteSword,
];

const PICKAXE_ORDER: [ItemKind; 7] = [
    ItemKind::WoodenPickaxe,
    ItemKind::GoldenPickaxe,
    ItemKind::StonePickaxe,
    ItemKind::CopperPickaxe,
    ItemKind::IronPickaxe,
    ItemKind::DiamondPickaxe,
    ItemKind::NetheritePickaxe,
];

const SHOVEL_ORDER: [ItemKind; 7] = [
    ItemKind::WoodenShovel,
    ItemKind::GoldenShovel,
    ItemKind::StoneShovel,
    ItemKind::CopperShovel,
    ItemKind::IronShovel,
    ItemKind::DiamondShovel,
    ItemKind::NetheriteShovel,
];

const HOE_ORDER: [ItemKind; 7] = [
    ItemKind::WoodenHoe,
    ItemKind::GoldenHoe,
    ItemKind::StoneHoe,
    ItemKind::CopperHoe,
    ItemKind::IronHoe,
    ItemKind::DiamondHoe,
    ItemKind::NetheriteHoe,
];

const HELMET_ORDER: [ItemKind; 8] = [
    ItemKind::LeatherHelmet,
    ItemKind::GoldenHelmet,
    ItemKind::CopperHelmet,
    ItemKind::ChainmailHelmet,
    ItemKind::IronHelmet,
    ItemKind::TurtleHelmet,
    ItemKind::DiamondHelmet,
    ItemKind::NetheriteHelmet,
];

const CHESTPLATE_ORDER: [ItemKind; 7] = [
    ItemKind::LeatherChestplate,
    ItemKind::CopperChestplate,
    ItemKind::GoldenChestplate,
    ItemKind::ChainmailChestplate,
    ItemKind::IronChestplate,
    ItemKind::DiamondChestplate,
    ItemKind::NetheriteChestplate,
];

const LEGGINGS_ORDER: [ItemKind; 7] = [
    ItemKind::LeatherLeggings,
    ItemKind::GoldenLeggings,
    ItemKind::CopperLeggings,
    ItemKind::ChainmailLeggings,
    ItemKind::IronLeggings,
    ItemKind::DiamondLeggings,
    ItemKind::NetheriteLeggings,
];

const BOOTS_ORDER: [ItemKind; 7] = [
    ItemKind::LeatherBoots,
    ItemKind::GoldenBoots,
    ItemKind::CopperBoots,
    ItemKind::ChainmailBoots,
    ItemKind::IronBoots,
    ItemKind::DiamondBoots,
    ItemKind::NetheriteBoots,
];

fn best_axe(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = AXE_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_sword(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = SWORD_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_pickaxe(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = PICKAXE_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_shovel(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = SHOVEL_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_hoe(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = HOE_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_helmet(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = HELMET_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_chestplate(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = CHESTPLATE_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_leggings(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = LEGGINGS_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}

fn best_boots(inventory: &Menu) -> Option<(usize, ItemKind)> {
    inventory
        .slots()
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let kind = item.kind();
            let priority = BOOTS_ORDER.iter().position(|k| *k == kind)?;
            Some((slot, kind, priority))
        })
        .max_by_key(|&(_, _, priority)| priority)
        .map(|(slot, kind, _)| (slot, kind))
}
