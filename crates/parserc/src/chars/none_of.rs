use std::marker::PhantomData;

use crate::{utils::FindChar, AsStr, Error, InputStream, Lookahead, Parser, ParserKind};

/// Parser created by [`none_of`] function.
pub struct NoneOf<T, I> {
    list: T,
    _marker: PhantomData<I>,
}

impl<T, I> Parser<I> for NoneOf<T, I>
where
    T: FindChar,
    I: InputStream + AsStr,
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
                    if !self.list.find(c) {
                        return Ok((input.split_at(c.len_utf8()), c));
                    } else {
                        return Err((input, Error::ParseFailed(ParserKind::NoneOf)));
                    }
                }

                if let Lookahead::BrokenPipe = input.lookahead(4).await {
                    return Err((input, Error::BrokenPipe(ParserKind::NoneOf)));
                }
            }
        }
    }
}

/// Recognizes one character and checks that it satisfies a predicate
pub fn none_of<I, T>(list: T) -> NoneOf<T, I>
where
    I: InputStream + AsStr,
    T: FindChar,
{
    NoneOf {
        list,
        _marker: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use futures::{executor::ThreadPool, task::SpawnExt};

    use crate::{Error, Parser, ParserKind};

    use super::*;

    #[futures_test::test]
    async fn test_none_of() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(none_of("é").parse("hello").await, Ok(("ello", 'h')));

            assert_eq!(
                none_of("é").parse("éhello").await,
                Err(("éhello", Error::ParseFailed(ParserKind::NoneOf)))
            );
        })
        .unwrap()
        .await
    }
}
