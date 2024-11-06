use std::marker::PhantomData;

use crate::{utils::FindChar, AsStr, Error, InputStream, Lookahead, Parser, ParserKind};

/// Parser created by [`one_of`] function.
pub struct OneOf<T, I> {
    list: T,
    _marker: PhantomData<I>,
}

impl<T, I> Parser<I> for OneOf<T, I>
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
                    if self.list.find(c) {
                        return Ok((input.split_at(c.len_utf8()), c));
                    } else {
                        return Err((input, Error::ParseFailed(ParserKind::OneOf)));
                    }
                }

                if let Lookahead::BrokenPipe = input.lookahead(4).await {
                    return Err((input, Error::BrokenPipe(ParserKind::OneOf)));
                }
            }
        }
    }
}

/// Recognizes one character and checks that it satisfies a predicate
pub fn one_of<I, T>(list: T) -> OneOf<T, I>
where
    I: InputStream + AsStr,
    T: FindChar,
{
    OneOf {
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
    async fn test_one_of() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(one_of("é").parse("éhello").await, Ok(("hello", 'é')));

            assert_eq!(
                one_of("é").parse("hello").await,
                Err(("hello", Error::ParseFailed(ParserKind::OneOf)))
            );

            assert_eq!(
                one_of("1234567890.".to_string()).parse("32.4").await,
                Ok(("2.4", '3'))
            );

            assert_eq!(
                one_of("é").parse("hello").await,
                Err(("hello", Error::ParseFailed(ParserKind::OneOf)))
            );
        })
        .unwrap()
        .await;
    }
}
