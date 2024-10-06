use teloxide::prelude::*;
use crate::loader::Error;

// Admin handler
pub async fn admin_handler(bot: Bot, message: Message) -> Result<(), Error> {
    bot.send_message(message.chat.id, "Salom, Admin!")
        .await?;
    Ok(())
}
