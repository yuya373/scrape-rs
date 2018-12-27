extern crate rayon;
extern crate reqwest;
extern crate zip;

use rayon::prelude::*;
use reqwest::{get, Result};
use std::io::prelude::*;
use zip::write::FileOptions;

struct Image {
    pub name: String,
    pub buf: Vec<u8>,
}

pub fn fetch(url: &str) -> Result<String> {
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

    for (i, image) in images.iter().enumerate() {
        zip.start_file(format!("{}-{}", i, image.name), options)?;
        zip.write(&image.buf)?;
    }

    zip.finish()?;
    Ok(())
}

pub fn save(title: String, srcs: Vec<String>) {
    let images: Vec<Image> = srcs
        .into_par_iter()
        .map(|src| {
            let mut buf: Vec<u8> = vec![];
            let mut resp = get(&src).expect(&format!("failed to get image: {:?}", src));
            resp.copy_to(&mut buf)
                .expect(&format!("failedo download image: {:?}", src));

            let path = std::path::Path::new(&src);
            let name = path.file_name().unwrap();
            Image {
                name: String::from(name.to_string_lossy()),
                buf,
            }
        })
        .collect();

    write_zip(&title, images).unwrap();
}
