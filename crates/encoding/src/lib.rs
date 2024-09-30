#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
#[cfg(not(feature = "std"))]
use std::prelude::v1::*;

/// The xml document byte stream's encoding.
#[derive(Debug, Clone)]
pub enum Encoding {
    Utf8,
    Utf16,
    Utf16LE,
    Utf16BE,
    Ascii,
    AsciiCompatible,
}
