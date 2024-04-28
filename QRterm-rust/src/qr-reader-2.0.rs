use qrcode_decoder_rust::{QRCodeDecoder, Color};
use image::{self, DynamicImage};
use std::fs::File;
use std::io::{self, BufReader};

pub fn read_qr_code(filename: Option<&str>) -> Result<String, String> {
    let decoder = QRCodeDecoder::new();

    if let Some(filename) = filename {
        let image = match image::open(filename) {
            Ok(img) => img,
            Err(err) => return Err(format!("Error opening image: {}", err)),
        };
        display_qr_code(&image)?;
        let gray = image.grayscale();


        let decoded_data = match decoder.decode_qr_code(&gray) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error decoding QR code: {}", err)),
        };
        return Ok(decoded_data);
    }


    println!("Scan a QR code (Ctrl+C to exit):");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read");

    let input = input.trim();
    match decoder.decode_qr_code_from_str(&input) {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error decoding QR code: {}", err)),
    }
}

fn display_qr_code(image: &DynamicImage) -> Result<(), std::io::Error> {
    println!("QR Code (placeholder):");
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            match pixel {
                image::Rgba(color) => {
                    let char = if color.0[0] > 128 { '#' } else { '.' };
                    print!("{}", char);
                }
                _ => unreachable!(),
            }
        }
        println!();
    }
    Ok(())
}

