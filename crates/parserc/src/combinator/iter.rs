use std::future::Future;

use crate::{inputs::InputStream, Parser, ParserKind};

/// A trait returns by [`iter`] combinator.
pub trait Generator<I> {
    /// Error type of this generator.
    type Error;

    /// Output type of this generator.
    type Output;

    /// Parse next `Output`.
    fn next(&mut self) -> impl Future<Output = Option<Self::Output>> + Send;

    /// Convert generator into stream.
    fn into_stream(self) -> I;
}

struct ParseIter<P, I> {
    parser: P,
    input: Option<I>,
}

impl<P, I, O, E> Generator<I> for ParseIter<P, I>
where
    P: Parser<I, Output = O, Error = E> + Send,
    I: InputStream,
{
    type Error = E;
    type Output = O;

    fn into_stream(mut self) -> I {
        self.input.take().unwrap()
    }
    fn next(&mut self) -> impl Future<Output = Option<Self::Output>> + Send {
        let input = self.input.take().unwrap();

        async move {
            let position = input.position();

            match self.parser.parse(input).await {
                Ok((i, o)) => {
                    // infinite loop check: the parser must always consume
                    if position == i.position() {
                        panic!("{}, infinite loop detected!!", ParserKind::Iter);
                    }

                    self.input = Some(i);
                    Some(o)
                }
                Err(_) => None,
            }
        }
    }
}

/// A combinator that loop over [`parser`](Parser) until the [`parser`](Parser) returns error.
///
/// `iter` has built-in infinite loop checking, so it is highly recommended to use `iter` function
/// rather than raw parser loops.
pub fn iter<P, I, O, E>(parser: P, input: I) -> impl Generator<I, Output = O, Error = E>
where
    P: Parser<I, Output = O, Error = E> + Send,
    I: InputStream,
{
    ParseIter {
        parser,
        input: Some(input),
    }
}

#[cfg(test)]
mod tests {

    use crate::Result;

    use super::*;

    async fn mock0<I>(input: I) -> Result<I, usize, ()>
    where
        I: InputStream,
    {
        Err((input, ()))
    }

    async fn mock1(input: &str) -> Result<&str, usize, ()> {
        let (_, input) = input.split_at(1);
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

        assert_eq!(mock0(gen.into_stream()).await, Err(("lo world", ())));
    }

    // #[futures_test::test]
    // async fn test_ctx() {
    //     #[derive(Debug)]
    //     struct Ctx(usize);

    //     impl Ctx {
    //         pub async fn update(&mut self, v: usize) {
    //             self.0 += v;
    //         }
    //     }

    //     async fn ctx_parser(input: (Ctx, &str)) -> Result<(Ctx, &str), (), ()> {
    //         let (mut ctx, input) = input;

    //         let mut gen = iter(mock1, input);

    //         for _ in 0..ctx.0 {
    //             ctx.update(gen.next().await.unwrap()).await;
    //         }

    //         Ok(((ctx, gen.into_stream()), ()))
    //     }

    //     let ((ctx, input), _) = map(ctx_parser, |_| 1)
    //         .parse((Ctx(3), "hello"))
    //         .await
    //         .unwrap();

    //     assert_eq!(input, "lo");

    //     assert_eq!(ctx.0, 6);
    // }
}
