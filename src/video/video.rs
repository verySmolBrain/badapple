// write
use std:: {
    fs::File,
    io::Write,
};

use ffmpeg::{ frame::Video, Error, software::scaling, format::Pixel, software::scaling::Flags, codec::context};
use termsize::Size;
use std::{thread, time};
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

    let size = Some(termsize::get()).unwrap();
    let Size { rows: term_rows, cols: term_cols } = size.unwrap();

    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width() / 3,
        decoder.height() / 3,
        Flags::BICUBIC,
    )?;

    let mut frame_index: i32 = 0;
    for (stream, packet) in ictx.packets() {
        if stream_index == stream.index() {
            frame_index += 1;
            decoder.send_packet(&packet)?;
            receive_and_write_frames(&mut decoder, &mut scaler, frame_index)?;
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    Ok(())
}

fn receive_and_write_frames(decoder: &mut ffmpeg::decoder::Video, scaler: &mut scaling::Context, frame_index: i32) -> Result<(), Error> {
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut frame = Video::empty();
        scaler.run(&decoded, &mut frame)?;
        print_frame(&frame, frame_index).unwrap();
    }

    Ok(())
}

fn print_frame(frame: &Video, index: i32) -> std::result::Result<(), std::io::Error> {
    let data = frame.data(frame.planes() - 1);
    let height = (frame.height() - 1) as usize;
    let width = (frame.width() - 1) as usize;

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let pixel = data[y * width + x];
            match pixel {
                0..= 25 => print!("."),
                26..= 50 => print!(":"),
                51..= 75 => print!("-"),
                76..= 100 => print!("="),
                101..= 125 => print!("+"),
                126..= 150 => print!("*"),
                151..= 175 => print!("$"),
                176..= 200 => print!("#"),
                _ => print!("@"),
            }
        }
        println!("a");
    }

    if index % 10 == 0 {
        let mut file = File::create(format!("data/frame{}.ppm", index))?;
        file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
        file.write_all(frame.data(0))?;
    }
    
    Ok(())
}