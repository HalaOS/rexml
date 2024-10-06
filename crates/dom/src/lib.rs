//! Document Object Model (DOM) Level 2 Core Specification Implementation for Rust.

// #![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(not(feature = "std"))]
// extern crate no_std_compat as std;
// #[cfg(not(feature = "std"))]
// use std::prelude::v1::*;

mod errors;
pub use errors::*;

mod qname;
pub use qname::*;

mod object;
pub use object::*;

mod arena;
pub use arena::*;
