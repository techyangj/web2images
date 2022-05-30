use clap::Parser;
use image::ImageFormat;
use std::{path::Path, ffi::OsStr};
use url::Url;

mod web2image;
use web2image::web2image;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long, default_value = "../images/1.jpg", validator = valid_filename)]
    #[clap(short, long, default_value = "./images/1.jpg")]
    output: String,

    #[clap(validator = valid_url)]
    url: String,

}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "png"  => Some(ImageFormat::Png),
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                _ => None,
            }
        })
}

fn valid_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_image_format(path);

    if parent.is_none() || ext.is_none() {
        return Err("File path must be exists and file must be jpg, jpeg or png".into());
    }
    Ok(())
}

fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("You must provide a valid url.".into()),
    }
}


fn main() {
    let args = Args::parse();

    println!("{:#?}",args);

    let format = get_image_format(Path::new(&args.output)).unwrap();

    web2image(&args.url, &args.output, format).unwrap();
}
