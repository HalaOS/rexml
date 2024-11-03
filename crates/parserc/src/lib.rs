//! `parsers` is a parser combinator library with a focus on safe paring, asynchronous compatible and large input data.

mod parsers;
pub use parsers::*;

pub mod combinator;
pub mod inputs;
