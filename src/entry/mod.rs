extern crate regex;
extern crate scraper;

use regex::Regex;
use scraper::Selector;

mod entry;
mod page;

pub use self::entry::*;
pub use self::page::*;

pub trait Scrapable {
    fn url(&self) -> &str;
    fn title_selector(&self) -> &str;
    fn image_selector(&self) -> &str;

    fn title(&self, document: &scraper::Html) -> String {
        let selector = Selector::parse(self.title_selector()).expect(&format!(
            "failed to parse selector: {:?}",
            self.title_selector()
        ));
        let title = document
            .select(&selector)
            .next()
            .expect(&format!(
                "failed to get title with selector: {:?}",
                selector
            ))
            .inner_html();
        let title_normalize_re = Regex::new(r"\r|\s|/").unwrap();
        title_normalize_re.replace_all(&title, "_").to_string()
    }

    fn image_srcs(&self, document: &scraper::Html) -> Vec<String> {
        let selector = Selector::parse(self.image_selector()).expect(&format!(
            "failed to parse selector: {:?}",
            self.image_selector()
        ));
        let mut image_srcs = vec![];
        for img in document.select(&selector) {
            match img.value().attr("src") {
                Some(src) => image_srcs.push(src.to_string()),
                None => {}
            }
        }
        image_srcs
    }
}
