use crate::{utils::FindChar, Error, InputStream, InputStreamUf8, Lookahead, Parser, ParserKind};

/// Recognizes one character and checks that it satisfies a predicate
pub fn none_of<I, T>(list: T) -> impl Parser<I, Error = Error, Output = char>
where
    I: InputStream + InputStreamUf8,
    T: FindChar + Clone + Send,
{
    move |mut input: I| {
        let list = list.clone();
        async move {
            loop {
                if let Some(c) = input.as_str().chars().next() {
                    if !list.find(c) {
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

#[cfg(test)]
mod tests {
    use crate::{Error, Parser, ParserKind};

    use super::*;

    #[futures_test::test]
    async fn test_none_of() {
        assert_eq!(none_of("é").parse("hello").await, Ok(("ello", 'h')));

        assert_eq!(
            none_of("é").parse("éhello").await,
            Err(("éhello", Error::ParseFailed(ParserKind::NoneOf)))
        );
    }
}
