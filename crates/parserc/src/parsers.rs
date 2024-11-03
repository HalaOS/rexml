//! Traits that all parserc `parsers` must implement.

use std::future::Future;

use crate::inputs::InputStream;

/// Result type returns by [`parse`](Parser::parse) function.
pub type Result<I, O, E> = std::result::Result<(I, O), (I, E)>;

/// All parserc parsers implement this trait
pub trait Parser<I>
where
    I: InputStream,
{
    type Error;

    type Output;

    /// A parser takes in input type, and returns a Result containing the output value, or an error
    fn parse(&self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>>;
}

impl<I, O, E, F, Fut> Parser<I> for F
where
    I: InputStream,
    F: Fn(I) -> Fut,
    Fut: Future<Output = Result<I, O, E>>,
{
    type Error = E;
    type Output = O;

    fn parse(&self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> {
        self(input)
    }
}
