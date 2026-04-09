use azalea::{
    BlockPos,
    ecs::query::With,
    entity::metadata,
    inventory::{self, Menu},
    local_player::Hunger,
    pathfinder::{
        self, custom_state::CustomPathfinderStateRef, mining::MiningCache, moves::MovesCtx,
        positions::RelBlockPos, world::CachedWorld,
    },
};

use crate::{
    commands::{CmdCtx, stop_all},
    item::inventory::sort_inventory,
};

pub async fn execute_stop(_args: &[&str], ctx: CmdCtx<'_>) {
    ctx.reply("Stopping");
    stop_all(ctx.bot, ctx.state).await;
}

pub async fn execute_disconnect(_args: &[&str], ctx: CmdCtx<'_>) {
    ctx.reply("Disconnecting");
    ctx.bot.disconnect();
}

pub async fn execute_health(_args: &[&str], ctx: CmdCtx<'_>) {
    ctx.reply(format!("health: {}", ctx.bot.health()));
}

pub async fn execute_hunger(_args: &[&str], ctx: CmdCtx<'_>) {
    let Hunger { food, saturation } = ctx.bot.hunger();
    ctx.reply(format!("hunger: {}, saturation: {}", food, saturation));
}

pub async fn execute_pos(_args: &[&str], ctx: CmdCtx<'_>) {
    let pos = ctx.bot.position();
    ctx.reply(format!("x: {:.1}, y: {:.1}, z: {:.1}", pos.x, pos.y, pos.z));
}

pub async fn execute_inventory(_args: &[&str], ctx: CmdCtx<'_>) {
    for item in ctx.bot.menu().slots() {
        if item.is_empty() {
            continue;
        };
        ctx.reply(format!("{} x{}", item.kind(), item.count()));
        for (kind, data) in item.component_patch().iter() {
            let Some(data) = data else {
                continue;
            };
            ctx.reply(format!("- {} {:?}", kind, data));
        }
    }
}

pub async fn execute_players(_args: &[&str], ctx: CmdCtx<'_>) {
    let player_entities = ctx
        .bot
        .nearest_entities_by::<(), With<metadata::Player>>(|_| true);
    let tablist = ctx.bot.tab_list();
    for entity in player_entities {
        let uuid = entity.uuid();
        ctx.reply(format!(
            "{} - {} ({:?})",
            entity.id(),
            tablist.get(&uuid).map_or("?", |p| p.profile.name.as_str()),
            uuid
        ));
    }
}

pub async fn execute_sortinv(_args: &[&str], ctx: CmdCtx<'_>) {
    sort_inventory(ctx.bot).await;
}

pub async fn execute_pathfinderstate(_args: &[&str], ctx: CmdCtx<'_>) {
    // TODO: bot stops when executed, wut??

    let pathfinder = {
        let pathfinder = ctx.bot.component::<azalea::pathfinder::Pathfinder>();
        pathfinder.clone()
    };

    if pathfinder.is_calculating {
        ctx.reply("Currently calculating path");
    }

    let executing_path = {
        let Some(executing_path) = ctx.bot.get_component::<azalea::pathfinder::ExecutingPath>()
        else {
            ctx.reply("Not currently executing a path");
            return;
        };
        executing_path.clone()
    };

    ctx.reply(format!(
        "is_path_partial: {}, path.len: {}, queued_path.len: {}",
        executing_path.is_path_partial,
        executing_path.path.len(),
        executing_path
            .queued_path
            .as_ref()
            .map(|q| q.len().to_string())
            .unwrap_or("n/a".to_string())
    ));
}

pub async fn execute_pathfindermoves(_args: &[&str], ctx: CmdCtx<'_>) {
    let Some(sender) = ctx.sender() else {
        ctx.reply("Couldn't determine sender".to_string());
        return;
    };
    let position = sender.position();
    let position = BlockPos::from(position);

    let mut edges = Vec::new();
    let cached_world = CachedWorld::new(ctx.bot.world(), position);
    let mining_cache = MiningCache::new(Some(Menu::Player(inventory::Player::default())));
    let custom_state = CustomPathfinderStateRef::default();

    pathfinder::moves::default_move(
        &mut MovesCtx {
            edges: &mut edges,
            world: &cached_world,
            mining_cache: &mining_cache,
            custom_state: &custom_state,
        },
        RelBlockPos::from_origin(position, position),
    );

    if edges.is_empty() {
        ctx.reply("No possible moves");
    } else {
        ctx.reply("Moves:");
        for edge in edges {
            ctx.reply(format!("{edge:?}"));
        }
    }
}
