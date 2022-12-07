extern crate termsize;

use std::env;
use crate::utils::error;
use crate::video::videoplayer;

mod video;
mod utils;

fn main() {
    let filename = env::args().nth(1).expect("Please provide a filename");

    match videoplayer::print_ascii_from_video(&filename) {
        Err(error) => {
            error::error_handler(error);
            std::process::exit(1);
        }
        Ok(()) => {
            std::process::exit(0);
        }
    }
}