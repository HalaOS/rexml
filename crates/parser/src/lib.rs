//! A XML document parser backed with nom.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
#[cfg(not(feature = "std"))]
use std::prelude::v1::*;

mod errors;
pub use errors::*;

pub mod block;
pub mod sax;
