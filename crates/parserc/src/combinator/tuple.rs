use std::future::Future;

use crate::{inputs::InputStream, Parser, Result};

/// A trait that the [`alt`] function argument must implement.
pub trait Seq<I>
where
    I: InputStream,
{
    type Error;

    type Output;
    /// A parser takes in input type, and returns a Result containing the output value, or an error
    fn parse(&self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>>;
}

struct SeqParser<C>(C);

impl<I, C> Parser<I> for SeqParser<C>
where
    C: Seq<I>,
    I: InputStream,
{
    type Error = C::Error;
    type Output = C::Output;

    fn parse(&self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> {
        self.0.parse(input)
    }
}

/// Applies a tuple of parsers one by one and returns their results as a tuple. There is a maximum of 21 parsers
pub fn tuple<I, T>(tuple: T) -> impl Parser<I, Output = T::Output, Error = T::Error>
where
    T: Seq<I>,
    I: InputStream,
{
    SeqParser(tuple)
}

macro_rules! seq_trait {
    ($header_a: ident $header_o: ident, $($tail_a: ident $tail_o: ident),+) => {

        impl<$header_a, $($tail_a),+ , I, $header_o, $($tail_o),+, E> Seq<I> for ($header_a, $($tail_a),+)
        where
            I: InputStream,
            $header_a: Parser<I,Output= $header_o, Error=E>,
            $($tail_a: Parser<I,Output= $tail_o, Error=E>),+
        {
            type Error = E;
            type Output = ($header_o, $($tail_o),+);

            fn parse(&self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> {
                #[allow(non_snake_case)]
                async move {

                    let ($header_a, $($tail_a),+) = self;

                    let (input, $header_o) = $header_a.parse(input).await?;

                    $(let (input,$tail_o) = $tail_a.parse(input).await?;)+

                    Ok((input, ($header_o, $($tail_o),+)))
                }
            }
        }

        seq_trait!($($tail_a $tail_o),+);
    };
    ($header_a: ident $header_o: ident) => {}
}

seq_trait!(
    A0 O0, A1 O1, A2 O2, A3 O3, A4 O4, A5 O5, A6 O6, A7 O7, A8 O8, A9 O9,
    A10 O10, A11 O11, A12 O12, A13 O13, A14 O14, A15 O15, A16 O16, A17 O17,
    A18 O18, A19 O19, A20 O20
);

#[cfg(test)]
mod tests {
    use crate::{inputs::InputStream, Parser};

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
        assert_eq!(
            tuple((mock0, mock1)).parse("hello world").await.unwrap(),
            ("hello world", (0, 1))
        );

        assert_eq!(
            tuple((mock2, mock1)).parse("hello world").await.unwrap(),
            ("hello world", (2, 1))
        );

        assert_eq!(
            tuple((mock2, mock0, mock1))
                .parse("hello world")
                .await
                .unwrap(),
            ("hello world", (2, 0, 1))
        );
    }
}
