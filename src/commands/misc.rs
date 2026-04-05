use crate::commands::CmdCtx;
use crate::stop_all;

pub async fn execute_stop(_args: &[&str], ctx: CmdCtx<'_>) {
    stop_all(ctx.bot, ctx.state).await;
}
