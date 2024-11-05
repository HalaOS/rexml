use crate::{InputStream, Parser};

struct Map<P, F> {
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
    ) -> impl std::future::Future<Output = crate::Result<I, Self::Output, Self::Error>> + Send {
        async move {
            self.parser
                .parse(input)
                .await
                .map(|(i, o)| (i, (self.map_f)(o)))
        }
    }
}

/// Maps a function on the result of a parser.
pub fn map<I, O1, O2, E, P, F>(parser: P, map_f: F) -> impl Parser<I, Output = O2, Error = E>
where
    I: InputStream,
    P: Parser<I, Output = O1, Error = E> + Send,
    F: Fn(O1) -> O2 + Send,
{
    Map { parser, map_f }
}
