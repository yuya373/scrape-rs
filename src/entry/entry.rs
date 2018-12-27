extern crate serde_derive;

use super::*;
use std::boxed::Box;
use std::io::Write;

#[derive(Deserialize)]
pub struct Entry {
    pub url: String,
    pub title_selector: String,
    pub image_selector: String,
}

impl Entry {
    pub fn get_page(&self) -> std::io::Result<Box<Scrapable + Send + Sync>> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        stdout.write(b"URL: ").unwrap();
        stdout.flush().unwrap();

        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(_) => {
                let url = buf.trim();

                println!("→ {:?}", url);
                Ok(Box::new(Page::new(
                    url.to_string(),
                    self.title_selector.clone(),
                    self.image_selector.clone(),
                )))
            }
            Err(e) => Err(e),
        }
    }
}
