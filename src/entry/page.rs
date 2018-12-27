use super::*;

pub struct Page {
    url: String,
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
}

impl Scrapable for Page {
    fn url(&self) -> &str {
        &self.url
    }

    fn title_selector(&self) -> &str {
        &self.title_selector
    }

    fn image_selector(&self) -> &str {
        &self.image_selector
    }
}
