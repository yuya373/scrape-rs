extern crate regex;
extern crate reqwest;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate toml;
extern crate zip;
#[macro_use]
extern crate log;
extern crate env_logger;

use rayon::ThreadPoolBuilder;
use scraper::Html;

mod config;
mod downloader;
mod entry;

use self::downloader::*;

fn main() {
    env_logger::init();

    let pool = ThreadPoolBuilder::new()
        .build()
        .expect("failed to build thread pool.");
    let config = config::get_config().expect("failed to load config.toml");

    for entry in config.entries {
        let cmd = if cfg!(target_os = "macos") {
            format!(
                "open -n -a \"Google Chrome\" --args --incognito '{}'",
                entry.url
            )
        } else {
            format!("google-chrome-stable --args --incognito '{}'", entry.url)
        };

        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap();

        loop {
            match entry.get_page() {
                Err(e) => println!("failed to read URL: {:?}", e),
                Ok(page) => {
                    if page.url().len() < 1 {
                        break;
                    }

                    pool.spawn(move || {
                        debug!("[document][start] {}", page.url());
                        let content = fetch(page.url())
                            .expect(&format!("failed to get content from: {:?}", page.url()));
                        debug!("[document][finish] {}", page.url());

                        let document = Html::parse_document(&content);
                        let title = page.title(&document);
                        let srcs = page.image_srcs(&document);

                        save(title, srcs);
                    })
                }
            }
        }
    }
}
