use std::future::Future;

use crate::{
    inputs::{InputStream, IntoInputStream},
    Parser,
};

/// Result type returns by [`next`](Generator::next) function.
pub type NextResult<I, O, E> = std::result::Result<Option<O>, (I, E)>;

/// A trait returns by `iter` combinator.
pub trait Generator<I>: IntoInputStream<Stream = I> {
    /// Error type of this generator.
    type Error;

    /// Output type of this generator.
    type Output;

    /// Parse next `Output`.
    fn next(&mut self) -> impl Future<Output = Option<Self::Output>>;
}

struct ParseIter<P, I> {
    parser: P,
    input: Option<I>,
}

impl<P, I, O, E> Generator<I> for ParseIter<P, I>
where
    P: Parser<I, Output = O, Error = E>,
    I: InputStream,
{
    type Error = E;
    type Output = O;

    fn next(&mut self) -> impl Future<Output = Option<Self::Output>> {
        let input = self.input.take().unwrap();

        async move {
            match self.parser.parse(input).await {
                Ok((i, o)) => {
                    self.input = Some(i);
                    Some(o)
                }
                Err(_) => None,
            }
        }
    }
}

impl<P, I, O, E> IntoInputStream for ParseIter<P, I>
where
    P: Parser<I, Output = O, Error = E>,
    I: InputStream,
{
    type Stream = I;

    fn into_input_stream(mut self) -> Self::Stream {
        self.input.take().unwrap()
    }
}

/// A combinator that loop call `parser` until returns error.
pub fn iter<P, I, O, E>(parser: P, input: I) -> impl Generator<I::Stream, Output = O, Error = E>
where
    P: Parser<I::Stream, Output = O, Error = E>,
    I: IntoInputStream,
{
    ParseIter {
        parser,
        input: Some(input.into_input_stream()),
    }
}

#[cfg(test)]
mod tests {

    use crate::{combinator::opt, Result};

    use super::*;

    async fn mock0<I>(input: I) -> Result<I::Stream, usize, ()>
    where
        I: IntoInputStream,
    {
        Err((input.into_input_stream(), ()))
    }

    async fn mock1(input: &str) -> Result<&str, usize, ()> {
        Ok((input, 1))
    }

    #[futures_test::test]
    async fn test_iter() {
        assert_eq!(iter(mock0, "hello world").next().await, None);

        // call many times.
        let mut gen = iter(mock1, "hello world");

        assert_eq!(gen.next().await, Some(1));
        assert_eq!(gen.next().await, Some(1));
        assert_eq!(gen.next().await, Some(1));

        // generator as InputStream

        assert_eq!(mock0(gen).await, Err(("hello world", ())));
    }

    #[futures_test::test]
    async fn test_ctx() {
        #[derive(Debug)]
        struct Ctx(usize);

        impl Ctx {
            pub async fn update(&mut self, v: usize) {
                self.0 += v;
            }
        }

        async fn ctx_parser(input: (Ctx, &str)) -> Result<(Ctx, &str), (), ()> {
            let (mut ctx, input) = input;

            let mut gen = iter(mock1, input);

            for _ in 0..ctx.0 {
                ctx.update(gen.next().await.unwrap()).await;
            }

            Ok(((ctx, gen.into_input_stream()), ()))
        }

        let ((ctx, input), op) = opt(ctx_parser).parse((Ctx(3), "hello")).await.unwrap();

        assert!(op.is_some());

        assert_eq!(input, "hello");

        assert_eq!(ctx.0, 6);
    }
}
