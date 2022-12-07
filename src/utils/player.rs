use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_PROP_FRAME_COUNT, CAP_PROP_FPS};
use crate::utils::error::BadAppleError;

struct Player {
    capture: VideoCapture,
    frame_count: f64,
    time_d: f64
}

impl Player {
    pub fn new(filename: &str) -> Result<Player, BadAppleError> {
        let capture = VideoCapture::from_file(filename, 0)?;
        let frame_count = capture.get(CAP_PROP_FRAME_COUNT)?;
        let time_d = 1.0 / capture.get(CAP_PROP_FPS)?;

        Ok(Player {
            capture,
            frame_count,
            time_d
        })
    }
}