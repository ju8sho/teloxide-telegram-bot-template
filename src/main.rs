mod commander;
mod loader;
mod config;
mod handlers;
mod models;
mod keyboards;

use log::debug;
use crate::loader::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Bot ishlamoqda...");

    run().await.expect("TODO: panic message");
}

