use std::future::Future;

use crate::{InputStream, Parser, Result};

/// Optional parser, will return None on inner parse returns Error.
pub struct Opt<P>(P);

impl<I, O, E, P> Parser<I> for Opt<P>
where
    P: Parser<I, Output = O, Error = E>,
    I: InputStream,
{
    type Error = E;

    type Output = Option<O>;

    fn parse(&mut self, input: I) -> impl Future<Output = Result<I, Self::Output, Self::Error>> {
        async move {
            match self.0.parse(input).await {
                Ok((input, i)) => Ok((input, Some(i))),
                Err((input, _)) => Ok((input, None)),
            }
        }
    }
}

/// Create a optional parser,will return None on inner parse returns Error.
pub fn opt<I, O, E, P>(parser: P) -> Opt<P>
where
    P: Parser<I, Output = O, Error = E>,
    I: InputStream,
{
    Opt(parser)
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::{executor::ThreadPool, task::SpawnExt};

    use crate::Result;

    #[derive(Debug)]
    struct Ctx(usize);

    impl Ctx {
        pub async fn update(&mut self, v: usize) {
            self.0 += v;
        }
    }

    async fn mock(input: &str) -> Result<&str, (), ()> {
        let mut ctx = Ctx(1);

        for _ in 0..ctx.0 {
            ctx.update(1).await;
        }

        Ok((input, ()))
    }

    #[futures_test::test]
    async fn test_opt() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(opt(mock).parse("hello").await, Ok(("hello", Some(()))));
        })
        .unwrap()
        .await;
    }
}
