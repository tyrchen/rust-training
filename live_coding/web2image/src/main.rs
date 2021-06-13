use std::{ffi::OsStr, path::Path};

use clap::{AppSettings, Clap};
use image::ImageFormat;
use url::Url;

mod web2image;
use web2image::web2image;

#[derive(Debug, Clap)]
#[clap(version = "0.1", author = "Tyr Chen <awesome@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// output file
    #[clap(short, long, default_value = "/tmp/snapshot.jpg", validator = valid_filename)]
    output: String,
    /// url to cpature
    #[clap(validator = valid_url)]
    url: String,
}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None,
            }
        })
}

fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("You must provide a valid url.".into()),
    }
}

/// "/tmp/abc.pdf" => "/tmp" exists, pdf (png | jpg | jpeg)
fn valid_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);
    // TODO: we have a small issue here. Have you found it?
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_image_format(path);

    if parent.is_none() || ext.is_none() {
        return Err("File path must be exists and file must be jpg, jpeg or png.".into());
    }
    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("{:#?}", opts);

    let format = get_image_format(Path::new(&opts.output)).unwrap();

    web2image(&opts.url, &opts.output, format).unwrap();
}
