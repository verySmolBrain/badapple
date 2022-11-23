use ffmpeg::{ frame::Video, Error, software::scaling::Context, format::Pixel, software::scaling::Flags};
use crate::utils::get_context;

pub fn print_ascii_from_video(filename: &str) -> Result<(), Error> {
    let mut ictx = get_context(&filename)?;

    let ictx_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(Error::StreamNotFound)?;

    let stream_index = ictx_stream.index();
    let mut decoder = ictx_stream.codec().decoder().video()?;
    
    for (stream, packet) in ictx.packets() {
        if stream_index != stream.index() {
            continue;
        }
        decoder.send_packet(&packet)?;
        receive_and_write_frames(&mut decoder)?;
        break; // remove later
    }
    //decoder.send_eof()?;

    Ok(())
}

fn receive_and_write_frames(decoder: &mut ffmpeg::decoder::Video ) -> Result<(), Error> {
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::GRAY8,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut frame = Video::empty();
        scaler.run(&decoded, &mut frame)?;
        println!("{:?}", frame.data(frame.planes() - 1));
    }

    Ok(())
}