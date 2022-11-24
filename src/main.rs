extern crate ffmpeg_next as ffmpeg;
extern crate termsize;

use std::env;

use crate::video::video::print_ascii_from_video;

mod video;
mod utils;

fn main() -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    let filename = env::args().nth(1).expect("Please provide a filename");
    println!("Opening {}", filename);

    print_ascii_from_video(&filename)?;

    Ok(())
}