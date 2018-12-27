extern crate regex;
extern crate reqwest;
extern crate scraper;

use crate::entry::Image;
use regex::Regex;
use reqwest::{get, Result};
use scraper::Selector;

pub struct Page {
    pub url: String,
    title_selector: String,
    image_selector: String,
}

impl Page {
    pub fn new(url: String, title_selector: String, image_selector: String) -> Page {
        Page {
            url,
            title_selector,
            image_selector,
        }
    }
    pub fn title(&self, document: &scraper::Html) -> String {
        let selector = Selector::parse(&self.title_selector).expect(&format!(
            "failed to parse selector: {:?}",
            self.title_selector
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

    pub fn images(&self, document: &scraper::Html) -> Result<Vec<Image>> {
        let image_srcs = self.image_srcs(document);
        self.fetch_images(image_srcs)
    }

    pub fn image_srcs(&self, document: &scraper::Html) -> Vec<String> {
        let selector = Selector::parse(&self.image_selector).expect(&format!(
            "failed to parse selector: {:?}",
            self.image_selector
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

    fn fetch_images(&self, image_srcs: Vec<String>) -> Result<Vec<Image>> {
        let mut images: Vec<Image> = vec![];
        for src in image_srcs {
            let mut buf: Vec<u8> = vec![];
            let mut resp = get(&src)?;
            resp.copy_to(&mut buf)?;
            let path = std::path::Path::new(&src);
            let name = path.file_name().unwrap();
            images.push(Image {
                name: String::from(name.to_string_lossy()),
                buf,
            });
        }
        Ok(images)
    }
}
