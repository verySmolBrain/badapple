use std::path::Path;

use ffmpeg::{format::context, Error};

pub fn get_context(filename: &str) -> Result<context::Input, Error> {
    let path = Path::new(filename);
    let ictx = ffmpeg::format::input(&path)?;

    Ok(ictx)
}