use crate::{utils::FindChar, Error, InputStream, InputStreamUf8, Lookahead, Parser, ParserKind};

/// Recognizes one character and checks that it satisfies a predicate
pub fn one_of<I, T>(list: T) -> impl Parser<I, Error = Error, Output = char>
where
    I: InputStream + InputStreamUf8,
    T: FindChar + Clone,
{
    move |mut input: I| {
        let list = list.clone();
        async move {
            loop {
                if let Some(c) = input.as_str().chars().next() {
                    if list.find(c) {
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

#[cfg(test)]
mod tests {
    use crate::{Error, Parser, ParserKind};

    use super::*;

    #[futures_test::test]
    async fn test_one_of() {
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
    }
}
