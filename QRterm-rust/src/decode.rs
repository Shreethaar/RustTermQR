use qrcode_decoder_rust::QRCodeDecoder;
use std::fs::File;
use std::io::Read;

pub fn read_qr_code_from_file(file_path: &str) -> Result<String, String> {
    let decoder = QRCodeDecoder::new();
    println!("Decoding QR code from file: {}", file_path);

    let mut file = File::open(file_path).map_err(|err| format!("Failed to open file: {}", err))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| format!("Failed to read file: {}", err))?;

    match decoder.decode_qr_code_from_vec(&buffer) {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error decoding QR code: {}", err)),
    }
}

