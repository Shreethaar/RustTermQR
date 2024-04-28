use qrcode_decoder_rust::{QRCodeDecoder, Color};

pub fn read_qr_code() -> Result<String, String> {
    let decoder = QRCodeDecoder::new();
    println!("Scan a QR code (Ctrl+C to exit):");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read");

    let input = input.trim();
    match decoder.decode_qr_code_from_str(&input) {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error decoding QR code: {}", err)),
    }
}
