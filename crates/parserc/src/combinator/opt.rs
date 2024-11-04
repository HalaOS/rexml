use std::future::Future;

use crate::{inputs::IntoInputStream, Parser, Result};

/// Optional parser, will return None on inner parse returns Error.
struct Opt<P>(P);

impl<I, O, E, P> Parser<I> for Opt<P>
where
    P: Parser<I, Output = O, Error = E>,
    I: IntoInputStream,
{
    type Error = E;

    type Output = Option<O>;

    fn parse(
        &mut self,
        input: I,
    ) -> impl Future<Output = Result<I::Stream, Self::Output, Self::Error>> {
        async move {
            match self.0.parse(input).await {
                Ok((input, i)) => Ok((input, Some(i))),
                Err((input, _)) => Ok((input, None)),
            }
        }
    }
}

/// Create a optional parser,will return None on inner parse returns Error.
pub fn opt<I, O, E, P>(parser: P) -> impl Parser<I, Output = Option<O>, Error = E>
where
    P: Parser<I, Output = O, Error = E>,
    I: IntoInputStream,
{
    Opt(parser)
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    use super::*;

    async fn mock<I>(input: I) -> Result<I, (), ()>
    where
        I: IntoInputStream,
    {
        let (input, _) = mock2(input).await?;
        let (input, _) = mock2(input).await?;
        Ok((input, ()))
    }

    async fn mock2<I>(input: I) -> Result<I, (), ()>
    where
        I: IntoInputStream,
    {
        Ok((input, ()))
    }

    #[futures_test::test]
    async fn test_opt() {
        opt(mock).parse("hello world").await.unwrap();
    }
}
