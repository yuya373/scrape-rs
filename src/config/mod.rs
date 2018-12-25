extern crate serde_derive;
extern crate toml;

use super::Entry;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use toml::de::Error;

#[derive(Deserialize)]
pub struct Config {
    pub entries: Vec<Entry>,
}

pub fn get_config() -> Result<Config, Error> {
    let current_dir = std::env::current_dir().unwrap();
    let filename = current_dir.join("config.toml");
    let file = File::open(filename).expect("config.toml does not exists.");

    let reader = BufReader::new(file);
    let mut buffer = String::new();

    for line in reader.lines() {
        buffer.push_str(&line.unwrap());
        buffer.push_str("\n");
    }

    toml::from_str(&buffer)
}
