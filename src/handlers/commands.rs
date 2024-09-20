use teloxide::Bot;
use teloxide::prelude::Requester;
use teloxide::types::User;
use teloxide::utils::command::BotCommands;
use crate::Error;
use crate::handlers::start_help::{start,help};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]

pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the bot.")]
    Start,
}


pub async fn process_command(bot: Bot, user: User, command: Command) -> Result<(), Error> {
    match command {
        Command::Start => start(bot, user).await,
        Command::Help => help(bot, user).await,
    }
}



