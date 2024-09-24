use teloxide::dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::dptree;
use teloxide::prelude::Update;
use teloxide::prelude::*;
use crate::handlers::commands::{process_command, Command};
use crate::handlers::blaclists::process_text_message;

pub fn creat_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync>> {
    Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())

        .branch( // filter for commands
            dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(process_command),
        )

        .branch( //filter for all message text return
            Message::filter_text().endpoint(process_text_message)
        )
}