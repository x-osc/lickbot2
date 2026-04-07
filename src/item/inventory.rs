use azalea::{
    container::ContainerHandle,
    inventory::{Menu, operations::SwapClick},
    prelude::*,
    registry::builtin::ItemKind,
};
use tracing::{debug, warn};

const AXE_HOTBAR_INDEX: usize = 0;
const SWORD_HOTBAR_INDEX: usize = 1;
const PICKAXE_HOTBAR_INDEX: usize = 2;
const SHOVEL_HOTBAR_INDEX: usize = 3;
const HOE_HOTBAR_INDEX: usize = 4;

pub fn sort_hotbar(bot: &Client) {
    let inventory_menu = bot.menu();
    let mut lazyinv = LazyInventory::new(bot);
    let hotbar_start = *inventory_menu.hotbar_slots_range().start();

    if let Some((best_slot, best_kind)) = best_axe(&inventory_menu)
        && best_slot != hotbar_start + AXE_HOTBAR_INDEX
    {
        debug!("Swapping {best_kind} at {best_slot} to slot {AXE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            // target slot is hotbar relative this isnt a mistake lmao
            target_slot: AXE_HOTBAR_INDEX as u8,
        });
    }
    if let Some((best_slot, best_kind)) = best_sword(&inventory_menu)
        && best_slot != hotbar_start + SWORD_HOTBAR_INDEX
    {
        debug!("Swapping {best_kind} at {best_slot} to slot {SWORD_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: SWORD_HOTBAR_INDEX as u8,
        });
    }
    if let Some((best_slot, best_kind)) = best_pickaxe(&inventory_menu)
        && best_slot != hotbar_start + PICKAXE_HOTBAR_INDEX
    {
        debug!("Swapping {best_kind} at {best_slot} to slot {PICKAXE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: PICKAXE_HOTBAR_INDEX as u8,
        });
    }
    if let Some((best_slot, best_kind)) = best_shovel(&inventory_menu)
        && best_slot != hotbar_start + SHOVEL_HOTBAR_INDEX
    {
        debug!("Swapping {best_kind} at {best_slot} to slot {SHOVEL_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: SHOVEL_HOTBAR_INDEX as u8,
        });
    }
    if let Some((best_slot, best_kind)) = best_hoe(&inventory_menu)
        && best_slot != hotbar_start + HOE_HOTBAR_INDEX
    {
        debug!("Swapping {best_kind} at {best_slot} to slot {HOE_HOTBAR_INDEX}");
        let Some(inventory_ref) = lazyinv.get() else {
            return;
        };
        inventory_ref.click(SwapClick {
            source_slot: best_slot as u16,
            target_slot: HOE_HOTBAR_INDEX as u8,
        });
    }
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
