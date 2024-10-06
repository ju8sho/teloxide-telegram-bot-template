
use teloxide::dptree;
use teloxide::dispatching::{Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::prelude::Requester;
use teloxide::types::Update;
use teloxide::utils::command::BotCommands;
use crate::commander::{Command, command_handlers};
use crate::config::Config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub async fn run() -> Result<(), Error> {

    let config = Config::new().await;

    let command_menu = Command::bot_commands();
    config.get_bot().set_my_commands(command_menu.clone()).await?;

    let commadn_handler = Update::filter_message()
        //commandant uchun handler daraxti
        .filter_command::<Command>()
        .endpoint(command_handlers);

    let handlers = dptree::entry()
        .branch(commadn_handler);

    Dispatcher::builder(config.get_bot().clone(), handlers)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}