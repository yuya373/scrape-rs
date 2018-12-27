extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate serde_derive;

pub struct Image {
    pub name: String,
    pub buf: Vec<u8>,
}

pub mod page;

use self::page::Page;
use std::io::Write;

#[derive(Deserialize)]
pub struct Entry {
    pub url: String,
    pub title_selector: String,
    pub image_selector: String,
    #[serde(default = "Entry::default_pages")]
    pub pages: Vec<String>,
}

impl Entry {
    pub fn default_pages() -> Vec<String> {
        vec![]
    }

    pub fn get_page(&self) -> std::io::Result<Page> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        stdout.write(b"URL: ").unwrap();
        stdout.flush().unwrap();

        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(_) => {
                let url = buf.trim();

                println!("→ {:?}", url);
                Ok(Page::new(
                    url.to_string(),
                    self.title_selector.clone(),
                    self.image_selector.clone(),
                ))
            }
            Err(e) => Err(e),
        }
    }
}
