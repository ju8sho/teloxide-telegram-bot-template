use rusqlite::ffi::sqlite3_callback;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn main_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::default()
        .resize_keyboard()
        .append_row(
        vec![
            KeyboardButton::new("Maxsulotlar")
        ]
    )
}