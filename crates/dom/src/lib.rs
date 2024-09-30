//! A DOM like api implementation for rexml.

// #![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(not(feature = "std"))]
// extern crate no_std_compat as std;
// #[cfg(not(feature = "std"))]
// use std::prelude::v1::*;

mod errors;
pub use errors::*;

mod namespace;
pub use namespace::*;

mod qname;
pub use qname::*;

mod object;
pub use object::*;
