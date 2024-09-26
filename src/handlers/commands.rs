use teloxide::Bot;
use teloxide::prelude::Requester;
use teloxide::types::User;
use teloxide::utils::command::BotCommands;
use crate::Error;
use crate::handlers::start_help::{start,help};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]

pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the bot.")]
    Start,
}


pub async fn process_command(bot: Bot, user: User, command: Command) -> Result<(), Error> {
    match command {
        Command::Start => start(bot, user).await,
        Command::Help => help(bot, user).await,
    }
}

// use teloxide::prelude::*;
// use teloxide::utils::command::BotCommands;
// use std::env;
// use crate::data::database::{add_product_to_db, remove_product_from_db};
// use crate::keyboards::keyboard::main_keyboard;
// use crate::models::model::{Buyurtma, Mahsulot};
//
// #[derive(BotCommands, Clone)]
// pub struct Command {
//     start: String,
//     help: String,
//
// }
//
// #[derive(BotCommands, Clone)]
// #[command(description = "Mavjud buyruqlar:")]
// pub enum Command {
//     #[command(description = "Botni ishga tushirish.")]
//     start,
//     #[command(description = "Yordam xabarini ko'rsatish.")]
//     help,
//     #[command(description = "Buyurtma berish.")]
//     buyurtma(String),
//     #[command(description = "Mahsulot qo'shish. (Faqat admin uchun)")]
//     addproduct { nomi: String, narxi: u32 },
//     #[command(description = "Mahsulot o'chirish. (Faqat admin uchun)")]
//     removeproduct { id: u32 },
// }
//
// pub async fn process_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
//     let user_id = msg.from().map(|user| user.id.0).unwrap_or(0);
//     let admin_id = env::var("ADMIN_ID").expect("ADMIN_ID must be set").parse::<i64>().unwrap();
//     let is_admin = *user_id == admin_id;
//
//     match cmd {
//         Command::Start => {
//             bot.send_message(msg.chat.id, "Xush kelibsiz!")
//                 .reply_markup(main_keyboard())
//                 .await?;
//         }
//         Command::Help => {
//             bot.send_message(msg.chat.id, "Mavjud buyruqlar:\n/start - Botni ishga tushirish\n/help - Yordam xabarini ko'rsatish\n/buyurtma <matn> - Buyurtma berish").await?;
//         }
//         Command::Buyurtma(order_text) => {
//             bot.send_message(
//                 ChatId(admin_id),
//                 format!("Yangi buyurtma:\nFoydalanuvchi ID: {}\nBuyurtma: {}", user_id, order_text)
//             ).await?;
//             bot.send_message(msg.chat.id, "Buyurtmangiz qabul qilindi!").await?;
//         }
//         Command::AddProduct { nomi, narxi } => {
//             if is_admin {
//                 add_product_to_db(&nomi, narxi).await.unwrap();
//                 bot.send_message(msg.chat.id, "Mahsulot muvaffaqiyatli qo'shildi.").await?;
//             } else {
//                 bot.send_message(msg.chat.id, "Bu buyruq faqat admin uchun.").await?;
//             }
//         }
//         Command::RemoveProduct { id } => {
//             if is_admin {
//                 remove_product_from_db(id).await.unwrap();
//                 bot.send_message(msg.chat.id, "Mahsulot muvaffaqiyatli o'chirildi.").await?;
//             } else {
//                 bot.send_message(msg.chat.id, "Bu buyruq faqat admin uchun.").await?;
//             }
//         }
//     }
//
//     Ok(())
// }



