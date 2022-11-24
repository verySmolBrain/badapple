use std::path::Path;

use ffmpeg::{format::context, format::input, Error};

pub fn get_context(filename: &str) -> Result<context::Input, Error> {
    let path = Path::new(filename);
    let ictx = input(&path)?;

    Ok(ictx)
}