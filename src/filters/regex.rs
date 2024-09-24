use regex::Regex;

pub struct LinkFilter {
    regex: Regex
}

impl LinkFilter {
    pub fn new() -> Self {
        let regex = Regex::new(
            r"(https?://)?(www\.)?(t\.me|telegram\.me|telegram\.org)/[a-zA-Z0-9_]+|https?://[^\s/$.?#].[^\s]*|@[a-zA-Z0-9_]+_bot")
            .expect("Link filterlashda xatolik");
        Self {regex}
    }

    pub fn url_filter(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }
}