//! `parserc` is a parser combinator library with a focus on safe parsing, future compatible and large input data.

mod parsers;
pub use parsers::*;

mod inputs;
pub use inputs::*;

mod errors;
pub use errors::*;

pub mod bytes;
pub mod chars;
pub mod combinator;
pub mod utils;
