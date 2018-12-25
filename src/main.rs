extern crate regex;
extern crate reqwest;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
extern crate num_cpus;
extern crate threadpool;
extern crate toml;
extern crate zip;

mod config;
mod entry;

use reqwest::{get, Result};
use scraper::Html;
use std::io::prelude::*;
use threadpool::ThreadPool;
use zip::write::FileOptions;

use self::entry::{Entry, Image};

fn fetch(url: &str) -> Result<String> {
    get(url)?.text()
}

fn write_zip(title: &str, images: Vec<Image>) -> zip::result::ZipResult<()> {
    let current_dir = std::env::current_dir()?;
    let dir = current_dir.join("downloads");
    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{}.zip", title));
    let path = std::path::Path::new(&path);
    let file = std::fs::File::create(&path).unwrap();

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    let mut zip = zip::ZipWriter::new(file);

    for image in images {
        zip.start_file(image.name, options)?;
        zip.write(&image.buf)?;
    }

    zip.finish()?;
    Ok(())
}

fn main() {
    let n_cpus = num_cpus::get();
    let pool = ThreadPool::new(n_cpus);
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
                    if page.url.len() < 1 {
                        break;
                    }

                    pool.execute(move || {
                        let content = fetch(&page.url)
                            .expect(&format!("failed to get content from: {:?}", page.url));
                        let document = Html::parse_document(&content);
                        let title = page.title(&document);

                        match page.images(&document) {
                            Ok(images) => write_zip(&title, images).unwrap(),
                            Err(e) => panic!(e),
                        }
                    });
                }
            }
        }
    }

    pool.join();
    println!(
        "Queued: {}, Max Concurrency: {}, Panic: {},",
        pool.queued_count(),
        pool.max_count(),
        pool.panic_count(),
    )
}
