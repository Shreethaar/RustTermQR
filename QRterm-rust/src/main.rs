use image;
use rqrr;

static img = image::open("tests/data/github.gif")?.to_luma();
let mut img = rqrr::PreparedImage::prepare(img);
let grids = img.detect_grids();
assert_eq!(grids.len(), 1);

let (meta, content) = grids[0].decode()?;
assert_eq!(meta.ecc_level, 0);
assert_eq!(content, "https://github.com/WanzenBug/rqrr");

/*
 * mod qr_reader;
use qrcode_decoder_rust::{ErrorCorrectionLevel};
use std::io::{self, Write};

fn main() -> Result<(), String> {
    let mut continuous = true;
    let mut verbose = false;
    let mut error_correction_level = ErrorCorrectionLevel::L;  // Default level

    loop {
        println!("Enter options (c - continuous, v - verbose, e - error correction level, anything else to start):");
        let mut option = String::new();
        io::stdin().read_line(&mut option)?;
        let option = option.trim();

        match option.chars().next().unwrap() {
            'c' => continuous = true,
            'v' => verbose = true,
            'e' => {
                println!("Select error correction level (L, M, Q, H):");
                let mut level = String::new();
                io::stdin().read_line(&mut level)?;
                let level = level.trim().chars().next().unwrap();
                error_correction_level = match level {
                    'L' => ErrorCorrectionLevel::L,
                    'M' => ErrorCorrectionLevel::M,
                    'Q' => ErrorCorrectionLevel::Q,
                    'H' => ErrorCorrectionLevel::H,
                    _ => return Err(format!("Invalid error correction level: {}", level)),
                };
            },
            _ => break,
        }
    }

    loop {
        println!("Enter filename (or press Enter to read from stdin):");
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)?;
        let filename = filename.trim();

        match qr_reader::read_qr_code(filename.is_empty().then(|| Some
  IMPLEMENTATION OF QRCODE_DECODER_RUST
  */
