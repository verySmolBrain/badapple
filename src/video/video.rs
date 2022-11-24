use std::{fs::File, io::Write};

use ffmpeg::{ frame::Video, Error, software::scaling, format::Pixel, software::scaling::Flags, codec::context};
use termsize::Size;
use crate::utils::get_context;

pub fn print_ascii_from_video(filename: &str) -> Result<(), Error> {
    let mut ictx = get_context(&filename)?;

    let ictx_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(Error::StreamNotFound)?;

    let stream_index = ictx_stream.index();

    let context_decoder = context::Context::from_parameters(ictx_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    //
    let scaling_factor = 6;
    // get terminal size
    let size = Some(termsize::get()).unwrap();
    let Size { rows: term_rows, cols: term_cols } = size.unwrap();
    //

    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width() / scaling_factor,
        decoder.height() / scaling_factor,
        Flags::BICUBIC,
    )?;
    
    for (stream, packet) in ictx.packets() {
        if stream_index == stream.index() {
            decoder.send_packet(&packet)?;
            receive_and_write_frames(&mut decoder, &mut scaler)?;
            break; // remove later
        }
    }

    Ok(())
}

fn receive_and_write_frames(decoder: &mut ffmpeg::decoder::Video, scaler: &mut scaling::Context ) -> Result<(), Error> {
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut frame = Video::empty();
        scaler.run(&decoded, &mut frame)?;
        print_frame(&frame).unwrap();
    }

    Ok(())
}

fn print_frame(frame: &Video) -> std::result::Result<(), std::io::Error> {
    let data = frame.data(frame.planes() - 1);
    let height = (frame.height() - 1) as usize;
    let width = (frame.width() - 1) as usize;

    for y in 0..height {
        for x in 0..width {
            let pixel = data[y * width + x];
            match pixel {
                0..= 63 => print!(" "),
                64..= 127 => print!("."),
                128..= 191 => print!("*"),
                192..= 255 => print!("8"),
            }
        }
        println!("a");
    }

    let mut file = File::create(format!("data/frame1.ppm"))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    
    Ok(())
}