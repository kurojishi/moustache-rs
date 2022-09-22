extern crate clap;
extern crate opencv;

use clap::Parser;

use moustache::moustacher::Moustacher;

#[derive(Parser)]
#[clap(author, about, version)]
struct Cli {
    source: String,
    destination: String,
}

const CASCADE_BASE: &str = "/usr/share/opencv4/haarcascades";
const CASCADE_FACE: &str = "haarcascade_frontalface_default.xml";
const MOUSTACHE_PATH: &str = "static/mustache.png";

fn main() {
    let args = Cli::parse();
    let source = args.source;
    let destination = args.destination;
    let cascade_face: String = format!("{CASCADE_BASE}/{CASCADE_FACE}");
    let moustacher = match Moustacher::new(cascade_face, String::from(MOUSTACHE_PATH)) {
        Ok(moustacher) => moustacher,
        Err(err) => panic!("Failed to initialize moustacher: {:?}", err),
    };
    match moustacher.add_moustache_to_image(source, destination.clone()) {
        Ok(_) => println!("Saved new moustached image to {}", destination),
        Err(err) => panic!("Failed to moustachify image with erro {:?}", err),
    };
}
