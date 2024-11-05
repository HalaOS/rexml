use crate::{Error, InputStream, IntoInputStream, Lookahead, Parser, ParserKind};

struct Tag<T>(T);

impl<T, I> Parser<I> for Tag<T>
where
    I: IntoInputStream,
    T: AsRef<[u8]>,
{
    type Error = Error;
    type Output = ();

    fn parse(
        &mut self,
        input: I,
    ) -> impl std::future::Future<
        Output = crate::Result<<I as crate::IntoInputStream>::Stream, Self::Output, Self::Error>,
    > {
        let mut input = input.into_input_stream();
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
    T: AsRef<[u8]> + Clone,
{
    Tag(tag)
}

#[cfg(test)]
mod tests {
    use crate::{
        combinator::{map, select},
        Error, Parser, ParserKind,
    };

    use super::*;

    #[futures_test::test]
    async fn test_tag() {
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
    }
}
