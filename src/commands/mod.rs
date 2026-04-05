use azalea::client_chat::ChatPacket;
use azalea::ecs::query::With;
use azalea::entity::metadata::Player;
use azalea::player::GameProfileComponent;
use azalea::{Client, EntityRef};

use crate::State;

mod misc;
mod movement;

pub struct CmdCtx<'a> {
    pub bot: &'a Client,
    pub state: &'a State,
    pub chat: ChatPacket,
}

impl CmdCtx<'_> {
    pub fn reply(&self, message: impl Into<String>) {
        self.bot.chat(message);
    }

    pub fn sender(&self) -> Option<EntityRef> {
        let username = self.chat.sender()?;
        self.bot
            .any_entity_by::<&GameProfileComponent, With<Player>>(
                |profile: &GameProfileComponent| profile.name == username,
            )
    }
}

fn tokenize(input: &str) -> (&str, Vec<&str>) {
    let input = input.trim();
    let mut tokens = input.split_whitespace();
    let cmd = tokens.next().unwrap_or("");
    let args: Vec<&str> = tokens.collect();
    (cmd, args)
}

pub fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

pub async fn execute(input: &str, ctx: CmdCtx<'_>) {
    let input = input.trim();
    if input.is_empty() {
        ctx.reply("Please enter a command".to_string());
        return;
    }

    let (cmd, args) = tokenize(input);

    match cmd {
        "goto" => movement::execute_goto(&args, ctx).await,
        "stop" => misc::execute_stop(&args, ctx).await,
        other => ctx.reply(format!("Unknown command: {other}")),
    };
}
