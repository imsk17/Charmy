#[warn(unused_variables)]
extern crate dotenv;

use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommand};

mod bans;
mod misc;
mod utils;

#[derive(BotCommand)]
#[command(
rename = "lowercase",
description = "These commands are supported:",
parse_with = "split"
)]
enum Command {
    #[command(description = "For Help")]
    Help,
    #[command(description = "Check if the bot is working or not.")]
    Start,
    #[command(description = "Kick a user from a chat")]
    Kick,
    #[command(description = "Ban a user from a chat, i.e. /ban 10 m/s/d/h")]
    Ban,
    #[command(description = "Mute a user from a chat, i.e. /mute 10 m/s/d/h")]
    Mute,
    #[command(description = "Unmute a user from a chat")]
    Unmute,
    #[command(description = "Unban a user from a chat")]
    Unban,
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    let _ = match command {
        Command::Help => misc::help(&cx, Command::descriptions()).await,
        Command::Start => misc::start(&cx).await,
        Command::Mute => bans::mute_user(&cx).await,
        Command::Ban => bans::ban_user(&cx).await,
        Command::Kick => bans::kick_user(&cx).await,
        Command::Unmute => bans::unmute_user(&cx).await,
        Command::Unban => bans::unban_user(&cx).await,
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    // lets load env variables
    dotenv().ok();
    // run the bot
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Charmy Bot, La!");
    let bot = Bot::from_env();
    teloxide::commands_repl(bot.clone(), "Charmy", answer).await;
}
