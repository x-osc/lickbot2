use std::{collections::HashMap, sync::LazyLock};

use azalea::{
    inventory::{ItemStack, Menu},
    registry::builtin::ItemKind,
};

use crate::item::extensions::LickbotMenuExt;

pub fn best_weapon_in_hotbar(menu: &Menu) -> usize {
    let hotbar_slots = menu.hotbar_slots();

    hotbar_slots
        .iter()
        .enumerate()
        .max_by(|(_, item1), (_, item2)| get_dps(item1).total_cmp(&get_dps(item2)))
        .unwrap()
        .0
}

pub fn get_dps(item: &ItemStack) -> f64 {
    let (damage, attack_speed) = get_damage_and_attack_speed(item).unwrap_or((1., 4.));
    // cap attack speed because i-frames
    let attack_speed = attack_speed.min(2.);
    damage / (1. / attack_speed)
}

pub fn get_damage_and_attack_speed(item: &ItemStack) -> Option<(f64, f64)> {
    WEAPON_DAMAGE.get(&item.kind()).map(|f| *f)
}

/// damage and attack speed of each weapon in the game
/// https://minecraft.wiki/w/Damage#Dealing_damage
pub static WEAPON_DAMAGE: LazyLock<HashMap<ItemKind, (f64, f64)>> = LazyLock::new(|| {
    HashMap::from([
        (ItemKind::WoodenSword, (4., 1.6)),
        (ItemKind::GoldenSword, (4., 1.6)),
        (ItemKind::StoneSword, (5., 1.6)),
        (ItemKind::CopperSword, (5., 1.6)),
        (ItemKind::IronSword, (6., 1.6)),
        (ItemKind::DiamondSword, (7., 1.6)),
        (ItemKind::NetheriteSword, (8., 1.6)),
        //
        (ItemKind::WoodenAxe, (7., 0.8)),
        (ItemKind::GoldenAxe, (7., 1.)),
        (ItemKind::StoneAxe, (9., 0.8)),
        (ItemKind::CopperAxe, (9., 0.8)),
        (ItemKind::IronAxe, (9., 0.9)),
        (ItemKind::DiamondAxe, (9., 1.)),
        (ItemKind::NetheriteAxe, (10., 1.)),
        //
        (ItemKind::WoodenPickaxe, (2., 1.2)),
        (ItemKind::GoldenPickaxe, (2., 1.2)),
        (ItemKind::StonePickaxe, (3., 1.2)),
        (ItemKind::CopperPickaxe, (3., 1.2)),
        (ItemKind::IronPickaxe, (4., 1.2)),
        (ItemKind::DiamondPickaxe, (5., 1.2)),
        (ItemKind::NetheritePickaxe, (6., 1.2)),
        //
        (ItemKind::WoodenShovel, (2.5, 1.)),
        (ItemKind::GoldenShovel, (2.5, 1.)),
        (ItemKind::StoneShovel, (3.5, 1.)),
        (ItemKind::CopperShovel, (3.5, 1.)),
        (ItemKind::IronShovel, (4.5, 1.)),
        (ItemKind::DiamondShovel, (5.5, 1.)),
        (ItemKind::NetheriteShovel, (6.5, 1.)),
        //
        (ItemKind::WoodenHoe, (1., 1.)),
        (ItemKind::GoldenHoe, (1., 1.)),
        (ItemKind::StoneHoe, (1., 2.)),
        (ItemKind::CopperHoe, (1., 2.)),
        (ItemKind::IronHoe, (1., 3.)),
        (ItemKind::DiamondHoe, (1., 4.)),
        (ItemKind::NetheriteHoe, (1., 4.)),
        //
        (ItemKind::Trident, (9., 1.1)),
        (ItemKind::Mace, (6., 0.6)),
    ])
});
