use crate::{Error, InputStream, Lookahead, Parser, ParserKind};

struct Tag<T>(T);

impl<T, I> Parser<I> for Tag<T>
where
    I: InputStream,
    T: AsRef<[u8]> + Send,
{
    type Error = Error;
    type Output = ();

    fn parse(
        &mut self,
        mut input: I,
    ) -> impl std::future::Future<Output = crate::Result<I, Self::Output, Self::Error>> + Send {
        async move {
            loop {
                if input.len() < self.0.as_ref().len() {
                    if let Lookahead::BrokenPipe =
                        input.lookahead(self.0.as_ref().len() - input.len()).await
                    {
                        return Err((input, Error::BrokenPipe(ParserKind::Tag)));
                    }

                    continue;
                }

                if &input.slice()[..self.0.as_ref().len()] == self.0.as_ref() {
                    return Ok((input.split_at(self.0.as_ref().len()), ()));
                } else {
                    return Err((input, Error::ParseFailed(ParserKind::Tag)));
                }
            }
        }
    }
}

pub fn tag<T, I>(tag: T) -> impl Parser<I, Output = (), Error = Error>
where
    I: InputStream,
    T: AsRef<[u8]> + Clone + Send,
{
    Tag(tag)
}

#[cfg(test)]
mod tests {
    use futures::{executor::ThreadPool, task::SpawnExt};

    use crate::{
        combinator::{map, select},
        Error, Parser, ParserKind,
    };

    use super::*;

    #[futures_test::test]
    async fn test_tag() {
        let pool = ThreadPool::new().unwrap();

        pool.spawn(async {
            assert_eq!(tag("éhello").parse("éhello~~~").await, Ok(("~~~", ())));

            assert_eq!(
                tag("éhello").parse("hello~~~").await,
                Err(("hello~~~", Error::ParseFailed(ParserKind::Tag)))
            );

            assert_eq!(
                select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
                    .parse("json hello")
                    .await,
                Ok((" hello", 2))
            );

            assert_eq!(
                select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
                    .parse("xml hello")
                    .await,
                Ok((" hello", 1))
            );

            assert_eq!(
                select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
                    .parse("a hello")
                    .await,
                Err(("a hello", Error::ParseFailed(ParserKind::Tag)))
            );
        })
        .unwrap();
    }
}
