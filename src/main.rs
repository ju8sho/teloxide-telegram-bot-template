mod handlers;
mod models;
mod keyboards;
mod dispatcher;
mod data;
mod filters;

use log::{debug, error};
use teloxide::prelude::*;
use dispatcher::creat_schema;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().expect("token uqishda xatolik");

    pretty_env_logger::init();
    log::info!("Bot ishga tushdi...");

    let bot = Bot::from_env();

    let schema = creat_schema();

    Dispatcher::builder(bot, schema).build().dispatch().await;
    Ok(())
}


