use azalea::prelude::*;

use crate::commands::CmdCtx;

pub async fn execute_stop(_args: &[&str], ctx: CmdCtx<'_>) {
    ctx.bot.stop_pathfinding();
}
