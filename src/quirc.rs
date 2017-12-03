// Copyright (C) 2017 - Will Glozer. All rights reserved.

#![allow(dead_code, non_camel_case_types)]

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct quirc {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct quirc_code {
    pub corners:     [quirc_point; 4usize],
    pub size:        c_int,
    pub cell_bitmap: [u8; QUIRC_MAX_BITMAP],
}

#[repr(C)]
pub struct quirc_data {
    pub version:     c_int,
    pub ecc_level:   c_int,
    pub mask:        c_int,
    pub data_type:   c_int,
    pub payload:     [u8; QUIRC_MAX_PAYLOAD],
    pub payload_len: c_int,
    pub eci:         u32,
}

#[repr(C)]
pub struct quirc_point {
    pub x: c_int,
    pub y: c_int,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum quirc_decode_error_t {
    QUIRC_SUCCESS                 = 0,
    QUIRC_ERROR_INVALID_GRID_SIZE = 1,
    QUIRC_ERROR_INVALID_VERSION   = 2,
    QUIRC_ERROR_FORMAT_ECC        = 3,
    QUIRC_ERROR_DATA_ECC          = 4,
    QUIRC_ERROR_UNKNOWN_DATA_TYPE = 5,
    QUIRC_ERROR_DATA_OVERFLOW     = 6,
    QUIRC_ERROR_DATA_UNDERFLOW    = 7,
}

pub const QUIRC_MAX_BITMAP:  usize = 3917;
pub const QUIRC_MAX_PAYLOAD: usize = 8896;

pub const QUIRC_ECC_LEVEL_M:       c_int = 0;
pub const QUIRC_ECC_LEVEL_L:       c_int = 1;
pub const QUIRC_ECC_LEVEL_H:       c_int = 2;
pub const QUIRC_ECC_LEVEL_Q:       c_int = 3;

pub const QUIRC_DATA_TYPE_NUMERIC: c_int = 1;
pub const QUIRC_DATA_TYPE_ALPHA:   c_int = 2;
pub const QUIRC_DATA_TYPE_BYTE:    c_int = 4;
pub const QUIRC_DATA_TYPE_KANJI:   c_int = 8;

pub const QUIRC_ECI_ISO_8859_1:    u32 = 1;
pub const QUIRC_ECI_IBM437:        u32 = 2;
pub const QUIRC_ECI_ISO_8859_2:    u32 = 4;
pub const QUIRC_ECI_ISO_8859_3:    u32 = 5;
pub const QUIRC_ECI_ISO_8859_4:    u32 = 6;
pub const QUIRC_ECI_ISO_8859_5:    u32 = 7;
pub const QUIRC_ECI_ISO_8859_6:    u32 = 8;
pub const QUIRC_ECI_ISO_8859_7:    u32 = 9;
pub const QUIRC_ECI_ISO_8859_8:    u32 = 10;
pub const QUIRC_ECI_ISO_8859_9:    u32 = 11;
pub const QUIRC_ECI_WINDOWS_874:   u32 = 13;
pub const QUIRC_ECI_ISO_8859_13:   u32 = 15;
pub const QUIRC_ECI_ISO_8859_15:   u32 = 17;
pub const QUIRC_ECI_SHIFT_JIS:     u32 = 20;
pub const QUIRC_ECI_UTF_8:         u32 = 26;

extern "C" {
    pub fn quirc_version() -> *const c_char;

    pub fn quirc_new() -> *mut quirc;
    pub fn quirc_destroy(q: *mut quirc);
    pub fn quirc_resize(q: *mut quirc, w: c_int, h: c_int) -> c_int;

    pub fn quirc_begin(q: *mut quirc, w: *mut c_int, h: *mut c_int) -> *mut u8;
    pub fn quirc_end(q: *mut quirc);

    pub fn quirc_count(q: *const quirc) -> c_int;
    pub fn quirc_extract(q: *const quirc, index: c_int, code: *mut quirc_code);
    pub fn quirc_decode(code: *const quirc_code, data: *mut quirc_data) -> quirc_decode_error_t;
}
