
pub struct Blacklist {
    pub sozlar: Vec<String>
}

impl Blacklist {
    pub fn new() -> Self {
        Self {
            sozlar: vec![
                "sildeotiladi".to_string(),
                "kilo".to_string(),
                "narx".to_string(),
                "so'm".to_string(),
                "tel".to_string(),
                "telefon".to_string(),
                "un".to_string(),
                "kepak".to_string(),
                "kepek".to_string(),
                "Kunjara".to_string(),
                "velik".to_string(),
                "$".to_string(),
                "onangni".to_string(),
                "horami".to_string(),
                "maraz".to_string(),
                "itvacha".to_string(),
                "pidr".to_string(),
                "pidaraz".to_string()


            ]
        }
    }
}