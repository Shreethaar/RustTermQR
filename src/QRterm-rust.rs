use std::io;
use qrcode::QrCode;
use qr2term::render;

fn main() {
    println!("Enter your message to generate a QR code:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let code = QrCode::new(input.trim().as_bytes()).expect("Failed to generate QR code");
    let rendered = render(&code);
    println!("{}", rendered);

    let decoded_message = String::from_utf8_lossy(&code.to_bytes());
    println!("Decoded message: {}", decoded_message);
}

