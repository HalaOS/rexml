//! `parsers` is a parser combinator library with a focus on safe paring, asynchronous compatible and large input data.

mod parsers;
pub use parsers::*;

mod inputs;
pub use inputs::*;

pub mod combinator;
