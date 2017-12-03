// Copyright (C) 2017 - Will Glozer. All rights reserved.

use std::mem;
use std::os::raw::c_int;
use std::ptr;
use std::slice;
use super::quirc::*;
use self::quirc_decode_error_t::*;

/// A QR code.
#[derive(Debug)]
pub struct QrCode {
    pub version:   u8,
    pub ecc_level: EccLevel,
    pub data_type: DataType,
    pub eci:       ECI,
    pub payload:   Vec<u8>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum EccLevel {
    M, L, H, Q
}

#[derive(Eq, PartialEq, Debug)]
pub enum DataType {
    Numeric,
    Alpha,
    Byte,
    Kanji,
    Other(u8),
}

#[derive(Eq, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum ECI {
    ISO_8859_1,
    IBM437,
    ISO_8859_2,
    ISO_8859_3,
    ISO_8859_4,
    ISO_8859_5,
    ISO_8859_6,
    ISO_8859_7,
    ISO_8859_8,
    ISO_8859_9,
    WINDOWS_874,
    ISO_8859_13,
    ISO_8859_15,
    SHIFT_JIS,
    UTF_8,
    Other(u8),
}

#[derive(Debug)]
pub enum Error {
    Alloc,
    Short,
    Decode(u32),
}

/// QR code decoder
pub struct QrCoder(*mut quirc);

impl QrCoder {
    /// Create a new `QrCoder`.
    pub fn new() -> Result<QrCoder, Error> {
        unsafe {
            match quirc_new() {
                q if !q.is_null() => Ok(QrCoder(q)),
                _                 => Err(Error::Alloc),
            }
        }
    }

    /// Return an `Iterator` over any `QrCodes` detected in the image.
    pub fn codes(&mut self, src: &[u8], width: u32, height: u32) -> Result<Codes, Error> {
        unsafe {
            if quirc_resize(self.0, width as c_int, height as c_int) < 0 {
                return Err(Error::Alloc);
            }

            let mut width:  c_int = 0;
            let mut height: c_int = 0;

            let ptr = quirc_begin(self.0, &mut width, &mut height);
            let len = (width * height) as usize;

            if src.len() < len {
                return Err(Error::Short);
            }

            ptr::copy_nonoverlapping(src.as_ptr(), ptr, len);

            quirc_end(self.0);

            let count = quirc_count(self.0) as usize;
            Ok(Codes::new(count, &mut *self.0))
        }
    }
}

impl Drop for QrCoder {
    fn drop(&mut self) {
        unsafe {
            quirc_destroy(self.0);
        }
    }
}

impl QrCode {
    fn new(qd: &quirc_data) -> Self {
        let ptr = qd.payload.as_ptr();
        let len = qd.payload_len as usize;

        let mut payload = Vec::with_capacity(len);

        unsafe {
            let slice = slice::from_raw_parts(ptr, len);
            payload.extend_from_slice(slice);
        }

        QrCode{
            version:   qd.version as u8,
            ecc_level: qd.ecc_level.into(),
            data_type: qd.data_type.into(),
            eci:       qd.eci.into(),
            payload:   payload,
        }
    }
}

/// `QrCode` iterator.
pub struct Codes<'a> {
    count: usize,
    next:  usize,
    quirc: &'a mut quirc,
    code:  quirc_code,
    data:  quirc_data,
}

impl<'a> Codes<'a> {
    fn new(count: usize, quirc: &'a mut quirc) -> Self {
        Codes{
            count: count,
            next:  0,
            quirc: quirc,
            code:  unsafe { mem::zeroed() },
            data:  unsafe { mem::zeroed() },
        }
    }
}

impl<'a> Iterator for Codes<'a> {
    type Item = Result<QrCode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.count {
            return None;
        }

        let index  = self.next as c_int;
        self.next += 1;

        unsafe {
            quirc_extract(self.quirc, index, &mut self.code);
            let data = &mut self.data;
            Some(match quirc_decode(&self.code, data) {
                QUIRC_SUCCESS => Ok(QrCode::new(data)),
                err           => Err(Error::Decode(err as u32)),
            })
        }
    }
}

impl From<c_int> for EccLevel {
    fn from(n: c_int) -> Self {
        match n {
            QUIRC_ECC_LEVEL_M => EccLevel::M,
            QUIRC_ECC_LEVEL_L => EccLevel::L,
            QUIRC_ECC_LEVEL_H => EccLevel::H,
            QUIRC_ECC_LEVEL_Q => EccLevel::Q,
            _                 => panic!("unsupported ECC level {}", n),
        }
    }
}

impl From<c_int> for DataType {
    fn from(n: c_int) -> Self {
        match n {
            QUIRC_DATA_TYPE_NUMERIC => DataType::Numeric,
            QUIRC_DATA_TYPE_ALPHA   => DataType::Alpha,
            QUIRC_DATA_TYPE_BYTE    => DataType::Byte,
            QUIRC_DATA_TYPE_KANJI   => DataType::Kanji,
            _                       => DataType::Other(n as u8),
        }
    }
}

impl From<u32> for ECI {
    fn from(n: u32) -> Self {
        match n {
            QUIRC_ECI_ISO_8859_1  => ECI::ISO_8859_1,
            QUIRC_ECI_IBM437      => ECI::IBM437,
            QUIRC_ECI_ISO_8859_2  => ECI::ISO_8859_2,
            QUIRC_ECI_ISO_8859_3  => ECI::ISO_8859_3,
            QUIRC_ECI_ISO_8859_4  => ECI::ISO_8859_4,
            QUIRC_ECI_ISO_8859_5  => ECI::ISO_8859_5,
            QUIRC_ECI_ISO_8859_6  => ECI::ISO_8859_6,
            QUIRC_ECI_ISO_8859_7  => ECI::ISO_8859_7,
            QUIRC_ECI_ISO_8859_8  => ECI::ISO_8859_8,
            QUIRC_ECI_ISO_8859_9  => ECI::ISO_8859_9,
            QUIRC_ECI_WINDOWS_874 => ECI::WINDOWS_874,
            QUIRC_ECI_ISO_8859_13 => ECI::ISO_8859_13,
            QUIRC_ECI_ISO_8859_15 => ECI::ISO_8859_15,
            QUIRC_ECI_SHIFT_JIS   => ECI::SHIFT_JIS,
            QUIRC_ECI_UTF_8       => ECI::UTF_8,
            _                     => ECI::Other(n as u8),
        }
    }
}
