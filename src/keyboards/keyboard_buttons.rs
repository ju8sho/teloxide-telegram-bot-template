use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn elon_matni_holati_uchun() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new("Yangi e'lon ✍️")],
        vec![
            KeyboardButton::new("Oxirgi e'londan nusxa 📋"),
            KeyboardButton::new("Shablon tanlash 📑"),
        ],
    ])
        .resize_keyboard()
}


// E'lon turlari uchun replay keyboard yaratish funksiyasi
pub(crate) fn elon_turi_replay_keyboard() -> KeyboardMarkup {
    let elon_turlari = vec![
        "💼 Ish",
        "🌾 Qishloq xo'jaligi",
        "🏠 Ko'chmas mulk",
        "🚗 Transport",
        "🔌 Elektronika",
        "🛠 Xizmatlar",
        "🎨 Hobbiy va qo'shimcha faoliyatlar",
        "🐕🐑🐄 Hayvonlar",
        "Boshqalar",
    ];

    let mut tugmalar = vec![];

    for elon_turi in elon_turlari.chunks(2) { // Har bir qatorga 2 ta tugma joylashtirish
        let row = elon_turi.iter().map(|matn| {
            // Keyboard tugmalarni yaratish
            KeyboardButton::new(matn.to_string())
        }).collect::<Vec<_>>();

        tugmalar.push(row);
    }

    KeyboardMarkup::new(tugmalar).resize_keyboard()
}
