pub fn is_blaclist(text: &str) -> bool {
    let blacklist = vec![
        "sotiladi", "kilo", "narx", "so'm", "tel", "telefon",
         "un", "kepek","kepak", "kunjara", "velik", "$"
    ];

    for matn in &blacklist {
        if text.to_lowercase().contains(matn) {return true}
    }

    false
}