use teloxide::prelude::*;
use crate::filters::filter::Filters;

pub async fn process_text_message(bot: Bot, message: Message) -> Result<(), crate::Error> {
    let filters = Filters::new();

    if let Some(text) = &message.text() {
        if filters.reklama_spam(text) { // Agar qora toyxatdagi sozlar va url reklamalar bolsa
            bot.delete_message(message.chat.id, message.id).await?;
        } else {
            bot.send_message(message.chat.id, text.clone()).await?; //(echo) }
        }
    }
    Ok(())
}