use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Bot, Requester};
use teloxide::types::User;
use crate::keyboards::keyboard::main_keyboard;


pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn start(bot: Bot, user: User) -> Result<(), Error> {
    bot.send_message(user.id, "Hush kelibsiz")
        .reply_markup(main_keyboard())
        .await?;
    Ok(())
}


pub async fn help(bot: Bot, user: User) -> Result<(), Error> {
    bot.send_message(user.id, "These commands are supported:\n/start - Start the bot\n/help - Display this help message").await?;
    Ok(())
}