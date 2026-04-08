use azalea::ecs::query::With;
use azalea::entity::metadata::Player;
use azalea::player::GameProfileComponent;

use crate::FollowingData;
use crate::commands::CmdCtx;

pub async fn execute_pvp(args: &[&str], ctx: CmdCtx<'_>) {
    match args {
        [] => {
            let Some(sender) = ctx.sender() else {
                ctx.reply("Couldn't determine sender".to_string());
                return;
            };

            ctx.reply("Attacking you");

            ctx.state
                .following_data
                .lock()
                .replace(FollowingData::new(sender.clone()));
            ctx.state.pvp_target.lock().replace(sender);
        }
        [name] => {
            let Some(player) = ctx
                .bot
                .any_entity_by::<&GameProfileComponent, With<Player>>(
                    |profile: &GameProfileComponent| profile.name == args[0],
                )
            else {
                ctx.reply(format!("Player {} not found", args[0]));
                return;
            };

            ctx.reply(format!("Attacking {name}"));

            ctx.state
                .following_data
                .lock()
                .replace(FollowingData::new(player.clone()));
            ctx.state.pvp_target.lock().replace(player);
        }
        _ => {
            ctx.reply("expected: !pvp <player>");
        }
    }
}
