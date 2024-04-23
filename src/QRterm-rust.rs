use qrcode::QrCode;
use qr2term::render;

fn main() {
    let code=QrCode::new(b"test").unwrap();
    let rendered=render(&code);
    println!("{}",rendered);
}


