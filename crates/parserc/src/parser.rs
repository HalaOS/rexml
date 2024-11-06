use std::future::Future;

use crate::{InputStream, Result};

/// All parsers must implement this trait.
pub trait Parser<I>: Send {
    /// Error type of this parser.
    type Error;
    /// The production type of this parser.
    type Output;

    /// takes in input type, and returns a Result containing the output value, or an error
    fn parse(
        &mut self,
        input: I,
    ) -> impl Future<Output = Result<I, Self::Output, Self::Error>> + Send;
}

impl<I, O, E, F, Fut> Parser<I> for F
where
    I: InputStream,
    F: FnMut(I) -> Fut + Send,
    Fut: Future<Output = Result<I, O, E>> + Send,
{
    type Error = E;
    type Output = O;

    fn parse(
        &mut self,
        input: I,
    ) -> impl Future<Output = Result<I, Self::Output, Self::Error>> + Send {
        self(input)
    }
}
