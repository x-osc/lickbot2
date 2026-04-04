use azalea::BlockPos;
use azalea::ecs::prelude::*;
use azalea::pathfinder::goals::{BlockPosGoal, XZGoal, YGoal};
use azalea::prelude::*;
use azalea::{entity::metadata::Player, player::GameProfileComponent};

use crate::commands::{CmdCtx, is_number};

pub async fn execute_goto(args: &[&str], ctx: CmdCtx<'_>) {
    match args {
        [] => {
            let Some(sender) = ctx.sender() else {
                ctx.reply("Couldn't determine sender".to_string());
                return;
            };

            let pos: BlockPos = sender.position().up(0.5).into();
            ctx.reply(format!(
                "Going to your position: x={} y={} y={}",
                pos.x, pos.y, pos.z
            ));
            ctx.bot.goto(BlockPosGoal(pos)).await;
        }
        [name] if !is_number(name) => {
            let Some(player) = ctx
                .bot
                .any_entity_by::<&GameProfileComponent, With<Player>>(
                    |profile: &GameProfileComponent| profile.name == *name,
                )
            else {
                ctx.reply(format!("Player {name} not found"));
                return;
            };

            let pos: BlockPos = player.position().up(0.5).into();
            ctx.reply(format!(
                "Going to {name}'s position: x={} y={} z={}",
                pos.x, pos.y, pos.z
            ));
            ctx.bot.goto(BlockPosGoal(pos)).await;
        }
        [y] if is_number(y) => {
            let y = y.parse().unwrap();
            ctx.reply(format!("Going to y={y}"));
            ctx.bot.goto(YGoal { y }).await;
        }
        [x, z] if is_number(x) && is_number(z) => {
            let (x, z) = (x.parse().unwrap(), z.parse().unwrap());
            ctx.reply(format!("Going to x={x} z={z}"));
            ctx.bot.goto(XZGoal { x, z }).await;
        }
        [x, y, z] if is_number(x) && is_number(y) && is_number(z) => {
            let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
            ctx.reply(format!("Going to x={x} y={y} z={z}"));
            ctx.bot.goto(BlockPosGoal(BlockPos { x, y, z })).await;
        }
        _ => {
            ctx.reply("expected: !goto <player> | !goto <x> <z> | !goto <x> <y> <z>");
        }
    };
}
