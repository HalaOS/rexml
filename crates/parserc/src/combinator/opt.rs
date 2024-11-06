use std::future::Future;

use crate::{InputStream, Parser, Result};

/// Optional parser, will return None on inner parse returns Error.
struct Opt<P>(P);

impl<I, O, E, P> Parser<I> for Opt<P>
where
    P: Parser<I, Output = O, Error = E> + Send,
    I: InputStream,
{
    type Error = E;

    type Output = Option<O>;

    fn parse(
        &mut self,
        input: I,
    ) -> impl Future<Output = Result<I, Self::Output, Self::Error>> + Send {
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
    P: Parser<I, Output = O, Error = E> + Send,
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

    #[test]
    fn test_opt() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn(async {
            assert_eq!(opt(mock).parse("hello").await, Ok(("", Some(()))));
        })
        .unwrap();
    }
}
