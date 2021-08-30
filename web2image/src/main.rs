use std::{ffi::OsStr, path::Path};

use clap::{AppSettings, Clap};
use url::Url;

mod web2image;
use web2image::web2image;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Gang Yin <hahayygg@coldmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// output file
    #[clap(short, long, default_value = "./test.png", validator = validate_filepath)]
    output: String,

    /// url to capture
    #[clap(validator = validate_url)]
    url: String,
}

fn validate_filepath(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    let parent = path.parent().and_then(|p| p.exists().then(|| p));
    let extension = path.extension()
                                .and_then(|ext| OsStr::to_str(ext))
                                .and_then(|ext| {
                                    match ext.to_lowercase().as_str() {
                                        "png" | "jpg" | "jpeg" => Some(()),
                                        _ => None,
                                    }
                                });
    if parent.is_some() && extension.is_some() {
        return Ok(());
    }
    Err("file path must exist and the file must be png, jpg or jpeg".into())
}

fn validate_url(url: &str) -> Result<(), String> {
    if let Ok(_) = Url::parse(url) {
        return Ok(())
    }
    Err("invalid url".into())
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);

    web2image(&opts.url, &opts.output);
}
