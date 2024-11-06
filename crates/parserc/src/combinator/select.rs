use std::future::Future;

use crate::{InputStream, Parser, Result};

/// A trait that used by [`select`] combinator.
pub trait Choice<I>
where
    I: InputStream,
{
    /// Error type returns by this trait.
    type Error;

    /// Return type when calling [`parse`](Choice::parse) function succeeds
    type Output;

    /// A parser takes in input type, and returns a Result containing the output value, or an error
    fn parse(&mut self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>>;
}

/// Parser created by [`select`] function.
pub struct ChoiceParser<C>(C);

impl<I, C> Parser<I> for ChoiceParser<C>
where
    C: Choice<I>,
    I: InputStream,
{
    type Error = C::Error;
    type Output = C::Output;

    fn parse(&mut self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> {
        self.0.parse(input)
    }
}

/// Tests a list of parsers one by one until one succeeds.
pub fn select<I, C>(choice: C) -> ChoiceParser<C>
where
    C: Choice<I>,
    I: InputStream,
{
    ChoiceParser(choice)
}

macro_rules! choice_trait {
    ($header: ident, $($tail: ident),+) => {
        choice_trait_impl!($header, $($tail),+);
        choice_trait!($($tail),+);
    };
    ($header: expr) => {}
}

macro_rules! choice_trait_impl {
    ($($ty: ident),+) => {
        impl<$($ty),+, I, O, E> Choice<I> for ($($ty),+)
        where
            I: InputStream + Clone,
            O: Send,
            $($ty: Parser<I, Output = O, Error = E> + Send),+
        {
            type Error = E;
            type Output = O;

            fn parse(&mut self, input: I) -> impl Future<Output = Result<I,O, E>>
            {
                async move {
                    #[allow(non_snake_case)]
                    let ($($ty),+) = self;
                    choice_parse_impl!(input, $($ty),+)
                }
            }
        }
    };
}

macro_rules! choice_parse_impl {
    ($input:expr, $header:ident, $($tail: ident),+) => ({

        if let Ok(r) = $header.parse($input.clone()).await {
            return Ok(r);
        }

        choice_parse_impl!($input, $($tail),+)
    });
    ($input:expr, $header: ident) => ({
        let r = $header.parse($input).await?;

        Ok(r)
    });
}

choice_trait!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20
);

#[cfg(test)]
mod tests {
    use crate::{combinator::map, Parser};

    use super::*;

    async fn mock0<I>(input: I) -> Result<I, usize, ()>
    where
        I: InputStream,
    {
        Err((input, ()))
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

    async fn mock3(input: &str) -> Result<&str, usize, ()> {
        select((mock0, mock1, mock0, mock2)).parse(input).await
    }

    #[futures_test::test]
    async fn test_select() {
        assert_eq!(mock3("Hello").await.unwrap(), ("Hello", 1));

        assert_eq!(
            select((mock0, map((mock2, mock1), |(a, b)| a + b), mock0, mock1))
                .parse("Hello")
                .await
                .unwrap(),
            ("Hello", 3)
        );
    }
}
