use teloxide::prelude::*;
use crate::config::Config;
use crate::handlers::admin::admin_handler;
use crate::loader::Error;


pub async fn start_handler(bot: Bot, msg: Message, config: &Config) -> Result<(), Error> {

    let user_id = msg.from.clone().map(|user| user.id.0 as i64).unwrap_or(0);
    if config.get_admin_id().contains(&user_id) {admin_handler(bot, msg).await?}
    else { bot.send_message(msg.chat.id, "Hush kelibsiz").await?; }
    Ok(())
}

pub async fn help_handler(bot: Bot, msg: Message) -> Result<(), Error> {
    bot.send_message(msg.chat.id, "bu yordam").await?;
    Ok(())
}