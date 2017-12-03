// Copyright (C) 2017 - Will Glozer. All rights reserved.

extern crate image;
extern crate qrcode;
extern crate quirc;

use std::str;
use image::Luma;
use qrcode::QrCode;
use quirc::QrCoder;

fn main() {
    let data  = b"Hello, World!";
    let code  = QrCode::new(&data).unwrap();
    let image = code.render::<Luma<u8>>().build();

    let mut quirc = QrCoder::new().unwrap();

    let width  = image.width();
    let height = image.height();
    let codes  = quirc.codes(&image, width, height).unwrap();

    for code in codes {
        let code = code.unwrap();
        let data = str::from_utf8(&code.payload).unwrap();
        println!("{:?}: {}", code, data);
    }
}
