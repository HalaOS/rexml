//! Traits that all parserc `parsers` must implement.

use std::future::Future;

use crate::inputs::InputStream;

/// Result type returns by [`parse`](Parser::parse) function.
pub type Result<I, O, E> = std::result::Result<(I, O), (I, E)>;

/// All parserc parsers implement this trait
pub trait Parser<I>: Send
where
    I: InputStream,
{
    /// Error type returns by [`parse`](Parser::parse) function.
    type Error;

    /// Parser generation target.
    type Output;

    /// A parser takes in input type, and returns a Result containing the output value, or an error
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

macro_rules! tuple_parser {
    ($header_a: ident $header_o: ident, $($tail_a: ident $tail_o: ident),+) => {

        impl<$header_a, $($tail_a),+ , I, $header_o, $($tail_o),+, E> Parser<I> for ($header_a, $($tail_a),+)
        where
            I: InputStream,
            $header_a: Parser<I,Output= $header_o, Error=E> + Send,
            $($tail_a: Parser<I,Output= $tail_o, Error=E> + Send),+,
            $header_o: Send,
            $($tail_o: Send),+,
        {
            type Error = E;
            type Output = ($header_o, $($tail_o),+);

            fn parse(&mut self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> + Send {
                #[allow(non_snake_case)]
                async move {

                    let ($header_a, $($tail_a),+) = self;

                    let (input, $header_o) = $header_a.parse(input).await?;

                    $(let (input,$tail_o) = $tail_a.parse(input).await?;)+

                    Ok((input, ($header_o, $($tail_o),+)))
                }
            }
        }

        tuple_parser!($($tail_a $tail_o),+);
    };
    ($header_a: ident $header_o: ident) => {}
}

tuple_parser!(
    A0 O0, A1 O1, A2 O2, A3 O3, A4 O4, A5 O5, A6 O6, A7 O7, A8 O8, A9 O9,
    A10 O10, A11 O11, A12 O12, A13 O13, A14 O14, A15 O15, A16 O16, A17 O17,
    A18 O18, A19 O19, A20 O20
);

#[cfg(test)]
mod tests {
    use futures::{executor::ThreadPool, task::SpawnExt};

    use super::*;
    async fn mock0<I>(input: I) -> Result<I, usize, ()>
    where
        I: InputStream,
    {
        Ok((input, 0))
    }

    async fn mock1<I>(input: I) -> Result<I, usize, ()>
    where
        I: InputStream,
    {
        Ok((input, 1))
    }

    async fn mock2<I>(input: I) -> Result<I, usize, ()>
    where
        I: InputStream,
    {
        Ok((input, 2))
    }

    #[futures_test::test]
    async fn test_tuple() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(
                (mock0, mock1).parse("hello world").await.unwrap(),
                ("hello world", (0, 1))
            );

            assert_eq!(
                (mock2, mock1).parse("hello world").await.unwrap(),
                ("hello world", (2, 1))
            );

            assert_eq!(
                (mock2, mock0, mock1).parse("hello world").await.unwrap(),
                ("hello world", (2, 0, 1))
            );
        })
        .unwrap()
        .await;
    }
}
