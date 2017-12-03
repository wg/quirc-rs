// Copyright (C) 2017 - Will Glozer. All rights reserved.

//! QR code decoder
//!
//! This crate provides a Rust interface to Daniel Beer's excellent
//! [quirc](https://github.com/dlbeer/quirc) QR code decoder.

mod api;
mod quirc;

pub use api::*;

#[cfg(test)]
extern crate image;
#[cfg(test)]
extern crate qrcode;

#[cfg(test)]
mod tests {
    use image::Luma;
    use qrcode::QrCode;
    use super::*;

    #[test]
    fn decode_one() {
        let data  = b"Hello, World!";
        let code  = QrCode::new(&data).unwrap();
        let image = code.render::<Luma<u8>>().build();

        let mut coder = QrCoder::new().unwrap();

        let width  = image.width();
        let height = image.height();

        let mut codes  = coder.codes(&image, width, height).unwrap();
        let code = codes.next().unwrap().unwrap();

        assert_eq!(1,              code.version);
        assert_eq!(EccLevel::M,    code.ecc_level);
        assert_eq!(DataType::Byte, code.data_type);
        assert_eq!(ECI::Other(0),  code.eci);
        assert_eq!(&data[..],      &code.payload[..]);

        assert!(codes.next().is_none());
    }
}
