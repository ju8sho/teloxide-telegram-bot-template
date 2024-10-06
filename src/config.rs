use teloxide::prelude::*;
use dotenv::dotenv;

pub struct Config {
    bot: Bot,
    admin_id: Vec<i64>
}

impl Config {
    pub async fn new() -> Self {
        dotenv().ok();

        let bot_token = std::env::var("TELOXIDE_TOKEN").expect("BOT_TOKEN topilmadi");
        let bot = Bot::new(bot_token);

        let admin_id: Vec<i64> = std::env::var("ADMIN_IDS")
            .expect("ADMIN_ID topilmadi")
            .split(',')
            .filter_map(|id| id.trim().parse().ok())
            .collect();

        Config {
            bot,
            admin_id
        }
    }

    pub fn get_bot(&self) -> &Bot { &self.bot }
    pub fn get_admin_id(&self) -> &Vec<i64> { &self.admin_id }

}
