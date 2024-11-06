use std::future::Future;

use crate::{InputStream, Parser};

/// Parser created by [`map`] function.
pub struct Map<P, F> {
    parser: P,
    map_f: F,
}

impl<I, O1, O2, E, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I, Output = O1, Error = E> + Send,
    I: InputStream,
    F: Fn(O1) -> O2 + Send,
{
    type Error = E;
    type Output = O2;

    fn parse(
        &mut self,
        input: I,
    ) -> impl Future<Output = crate::Result<I, Self::Output, Self::Error>> {
        async move {
            self.parser
                .parse(input)
                .await
                .map(|(i, o)| (i, (self.map_f)(o)))
        }
    }
}

/// Maps a function on the result of a parser.
pub fn map<I, O1, O2, E, P, F>(parser: P, map_f: F) -> Map<P, F>
where
    I: InputStream,
    P: Parser<I, Output = O1, Error = E> + Send,
    F: Fn(O1) -> O2 + Send,
{
    Map { parser, map_f }
}
