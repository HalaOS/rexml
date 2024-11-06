use std::{marker::PhantomData, usize};

use crate::{Error, InputStream, Lookahead, Parser, ParserKind};

/// Parser created by [`search`] function.
pub struct Search<F, I> {
    cond: F,
    _marker: PhantomData<I>,
}

impl<F, I> Parser<I> for Search<F, I>
where
    I: InputStream,
    F: Fn(usize, u8) -> bool + Clone + Send,
{
    type Error = Error;
    type Output = usize;

    fn parse(
        &mut self,
        mut input: I,
    ) -> impl std::future::Future<Output = crate::Result<I, Self::Output, Self::Error>> {
        async move {
            let mut offset = 0usize;
            loop {
                if input.len() - offset == 0 {
                    if let Lookahead::BrokenPipe = input.lookahead(1024).await {
                        return Err((input, Error::BrokenPipe(ParserKind::Search)));
                    }

                    continue;
                }

                if (self.cond)(offset, input.slice()[offset]) {
                    return Ok((input, offset));
                }

                offset += 1;
            }
        }
    }
}

/// Search along the input stream until cond returns false.
pub fn search<I, F>(cond: F) -> Search<F, I>
where
    I: InputStream,
    F: Fn(usize, u8) -> bool + Clone + Send,
{
    Search {
        cond,
        _marker: Default::default(),
    }
}

#[cfg(test)]
mod tests {

    use futures::{executor::ThreadPool, task::SpawnExt};

    use super::*;

    #[futures_test::test]
    async fn test_tag() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn_with_handle(async {
            assert_eq!(
                search(|_, v| { (v as char).is_whitespace() })
                    .parse("hello ~~~")
                    .await,
                Ok(("hello ~~~", 5))
            );

            assert_eq!(
                search(|_, v| { (v as char).is_whitespace() })
                    .parse("")
                    .await,
                Err(("", Error::BrokenPipe(ParserKind::Search)))
            );
        })
        .unwrap()
        .await;
    }
}
