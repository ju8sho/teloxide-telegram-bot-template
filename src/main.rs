mod handlers;
mod models;
mod keyboards;
mod dispatcher;
mod data;

use teloxide::prelude::*;
use teloxide::types::MenuButton::Commands;
use dispatcher::creat_schema;
use crate::handlers::commands::{answer, Command};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    dotenv::dotenv().ok();
    let bot = Bot::from_env();
    let bot2 = bot.clone();

    Command::repl(bot2, answer);

    let schema = creat_schema();

    Dispatcher::builder(bot, schema).build().dispatch().await;
    Ok(())
}


