use crate::filters::blacklist::Blacklist;
use crate::filters::regex::LinkFilter;

pub struct Filters {
    pub royxat: Blacklist,
    pub link_filter: LinkFilter
}

impl Filters {
    pub fn new() -> Self {
        Self {
            royxat: Blacklist::new(),
            link_filter: LinkFilter::new()
        }
    }

    pub fn is_blacklisted(&self, text: &str) -> bool {
        for soz in &self.royxat.sozlar {
            if text.to_lowercase().contains(soz) {return true}
        }
        false
    }

    pub fn is_url(&self, text: &str) -> bool {
        self.link_filter.url_filter(text)
    }

    pub fn reklama_spam(&self, text: &str) -> bool {
        self.is_blacklisted(text) || self.is_url(text)
    }

}