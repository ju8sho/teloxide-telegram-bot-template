use teloxide::Bot;
use teloxide::macros::BotCommands;
use teloxide::prelude::Message;
use crate::config::Config;
use crate::handlers::elon_yigish::elon_yaratish;
use crate::handlers::start_help::{help_handler, start_handler};
use crate::loader::Error;
use crate::handlers::elon_yigish::MyDialogue;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Mavjud buyruqlar:")]
pub enum Command {
    #[command(description = "botni ishga tushirish.")]
    Start,
    #[command(description = "yordam xabarini ko'rsatish.")]
    Help,
}


pub(crate) async fn command_handlers(bot: Bot, message: Message, cmd: Command, dialogue: MyDialogue) -> Result<(), Error> {
    let config = Config::new().await;
    match cmd {
        Command::Start => { start_handler(bot, message, &config).await }
        Command::Help => help_handler(bot, message).await,
    }
}