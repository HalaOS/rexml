use std::marker::PhantomData;

use crate::{AsStr, Error, InputStream, Lookahead, Parser, ParserKind};

/// Parser returns by [`satisfy`] function.
pub struct Satisfy<F, I> {
    cond: F,
    _marker: PhantomData<I>,
}

impl<F, I> Parser<I> for Satisfy<F, I>
where
    I: InputStream + AsStr,
    F: Fn(char) -> bool + Clone + Send,
{
    type Error = Error;
    type Output = char;

    fn parse(
        &mut self,
        mut input: I,
    ) -> impl std::future::Future<Output = crate::Result<I, Self::Output, Self::Error>> {
        async move {
            loop {
                if let Some(c) = input.as_str().chars().next() {
                    if (self.cond)(c) {
                        return Ok((input.split_at(c.len_utf8()), c));
                    } else {
                        return Err((input, Error::ParseFailed(ParserKind::Safisfy)));
                    }
                }

                if let Lookahead::BrokenPipe = input.lookahead(4).await {
                    return Err((input, Error::BrokenPipe(ParserKind::Safisfy)));
                }
            }
        }
    }
}

/// Recognizes one character and checks that it satisfies a predicate
pub fn satisfy<I, F>(cond: F) -> Satisfy<F, I>
where
    I: InputStream + AsStr,
    F: Fn(char) -> bool + Clone + Send,
{
    Satisfy {
        cond,
        _marker: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use futures::{executor::ThreadPool, task::SpawnExt};

    use crate::{Error, Parser, ParserKind};

    use super::satisfy;

    #[futures_test::test]
    async fn test_satisfy() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(
                satisfy(|c| c == 'é').parse("éhello").await,
                Ok(("hello", 'é'))
            );

            assert_eq!(
                satisfy(|c| c == 'é').parse("hello").await,
                Err(("hello", Error::ParseFailed(ParserKind::Safisfy)))
            );
        })
        .unwrap()
        .await;
    }
}
