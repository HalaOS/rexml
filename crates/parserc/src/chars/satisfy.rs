use crate::{Error, InputStream, InputStreamUf8, Lookahead, Parser, ParserKind};

/// Recognizes one character and checks that it satisfies a predicate
pub fn satisfy<I, F>(cond: F) -> impl Parser<I, Error = Error, Output = char>
where
    I: InputStream + InputStreamUf8,
    F: Fn(char) -> bool + Clone,
{
    move |mut input: I| {
        let cond = cond.clone();

        async move {
            loop {
                if let Some(c) = input.as_str().chars().next() {
                    if cond(c) {
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

#[cfg(test)]
mod tests {
    use crate::{Error, Parser, ParserKind};

    use super::satisfy;

    #[futures_test::test]
    async fn test_satisfy() {
        assert_eq!(
            satisfy(|c| c == 'é').parse("éhello").await,
            Ok(("hello", 'é'))
        );

        assert_eq!(
            satisfy(|c| c == 'é').parse("hello").await,
            Err(("hello", Error::ParseFailed(ParserKind::Safisfy)))
        );
    }
}
