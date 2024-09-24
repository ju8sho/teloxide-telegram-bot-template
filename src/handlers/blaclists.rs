use teloxide::prelude::*;
use crate::filters::blacklist::is_blaclist;

pub async fn process_text_message(bot: Bot, message: Message) -> Result<(), crate::Error> {
    if let Some(text) = &message.text() {
        if is_blaclist(text) { // Agar qora royxatda qoralama soz bolsa o'chiramiz
            bot.delete_message(message.chat.id, message.id).await?;
        }
    }
    Ok(())
}