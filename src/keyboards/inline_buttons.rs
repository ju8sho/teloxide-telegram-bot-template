use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

// E'lon turlari uchun inline keyboard yaratish funksiyasi
pub(crate) fn elon_turi_inline() -> InlineKeyboardMarkup {
    let elon_turlari = vec![
        ("Ish", "💼, 🔍"),
        ("Qishloq xo'jaligi", "🌾, 🚜, 🥕, 🍅"),
        ("Ko'chmas mulk", "🏠"),
        ("Transport", "🚗"),
        ("Elektronika", "🔌, 💻, 📱"),
        ("Xizmatlar", "🛠, 🔧"),
        ("Hobbiy va qo'shimcha faoliyatlar", "🎨, 🎮, 🎼"),
        ("Hayvonlar", "🐕🐑🐄"),
        ("Boshqalar", "")
    ];

    let mut tugmalar = vec![];

    for elon_turi in elon_turlari.chunks(2) { // Har bir qatorga 2 ta tugma joylashtirish
        let row = elon_turi.iter().map(|(matn, emoji)| {
            // Tugma yaratish, emojilarni tugma matnida biriktirish
            InlineKeyboardButton::callback(format!("{} {}", emoji, matn), matn.to_string())
        }).collect::<Vec<_>>();

        tugmalar.push(row);
    }

    InlineKeyboardMarkup::new(tugmalar)
}