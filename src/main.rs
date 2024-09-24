mod handlers;
mod models;
mod dispatcher;
mod data;
mod filters;
mod keyboards;

use teloxide::prelude::*;
use dispatcher::creat_schema;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    dotenv::dotenv().ok();
    let bot = Bot::from_env();

    let schema = creat_schema();

    Dispatcher::builder(bot, schema).build().dispatch().await;
    Ok(())
}


