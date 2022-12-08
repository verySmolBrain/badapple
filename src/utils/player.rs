use crate::utils::error::BadAppleError;
use crate::utils::converter;

use opencv::prelude::*;
use opencv::{ videoio, imgproc, core };
use std::{ io, time };
use std::io::Write; // WHY?
use termsize::Size;

pub struct Player {
    capture: videoio::VideoCapture,
    frame_count: f64,
    time_d: f64
}

impl Player {
    pub fn new(filename: &str) -> Result<Player, BadAppleError> {
        let capture = videoio::VideoCapture::from_file(filename, 0)?;
        let frame_count = capture.get(videoio::CAP_PROP_FRAME_COUNT)?;
        let time_d = 1.0 / capture.get(videoio::CAP_PROP_FPS)?;

        Ok(Player {
            capture,
            frame_count,
            time_d
        })
    }

    pub fn play_to_stdout(&mut self) -> Result<(), BadAppleError> {
        let mut out = io::stdout().lock();

        for _ in 0..self.frame_count as i32 {
            let start = time::SystemTime::now();

            let mut frame = Mat::default();
            let mut resized_frame = Mat::default();

            self.capture.read(&mut frame)?;
            let Size { cols: width, rows: height } = termsize::get().unwrap();

            imgproc::resize(
                &frame,
                &mut resized_frame,
                core::Size::new(
                    i32::from(width - 1),
                    i32::from(height - 1)
                ),
                0.0,
                0.0,
                imgproc::INTER_AREA
            )?;

            let output = converter::convert_frame(&resized_frame)?;
            
            write!(out, "{esc}c", esc = 27 as char)?;
            write!(out, "{}", output)?;

            let elapsed = start.elapsed().unwrap().as_secs_f64();
            if elapsed < self.time_d {
                std::thread::sleep(time::Duration::from_secs_f64(self.time_d - elapsed));
            }
        }

        Ok(())
    }
}