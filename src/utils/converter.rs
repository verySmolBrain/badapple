use crate::utils::error::BadAppleError;
use opencv::prelude::*;
use opencv::core;

pub fn convert_frame(frame: &Mat) -> Result<String, BadAppleError> {
    let mut output = String::default(); // new vs default?

    for i in 0..frame.rows() {
        for j in 0..frame.cols() {
            let pixel = frame.at_2d::<core::Vec3b>(i, j)?;
            let ascii_pixel = convert_pixel(pixel)?;
            output.push(ascii_pixel);
        }
        output.push('\n');
    }

    Ok(output)
}

fn convert_pixel(pixel: &core::Vec3b) -> Result<char, BadAppleError> {
    let b = pixel[0];
	let g = pixel[1];
	let r = pixel[2];

    // https://stackoverflow.com/questions/596216/formula-to-determine-perceived-brightness-of-rgb-color
    let brightness = 0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b);

    match brightness as i32 {
        0 => Ok(' '),
        1..= 25 => Ok('.'),
        26..= 50 => Ok(':'),
        51..= 75 => Ok('-'),
        76..= 100 => Ok('='),
        101..= 125 => Ok('+'),
        126..= 150 => Ok('*'),
        151..= 175 => Ok('$'),
        176..= 200 => Ok('#'),
        _ => Ok('@'),
    }
}