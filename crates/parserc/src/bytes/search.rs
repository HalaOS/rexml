use std::usize;

use crate::{Error, InputStream, IntoInputStream, Lookahead, Parser, ParserKind};

/// Search along the input stream until cond returns false.
pub fn search<I, F>(cond: F) -> impl Parser<I, Output = usize, Error = Error>
where
    I: IntoInputStream,
    F: Fn(usize, u8) -> bool + Clone,
{
    move |input: I| {
        let cond = cond.clone();
        let mut input = input.into_input_stream();
        async move {
            let mut offset = 0usize;
            loop {
                if input.len() - offset == 0 {
                    if let Lookahead::BrokenPipe = input.lookahead(1024).await {
                        return Err((input, Error::BrokenPipe(ParserKind::Search)));
                    }

                    continue;
                }

                if cond(offset, input.slice()[offset]) {
                    return Ok((input, offset));
                }

                offset += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[futures_test::test]
    async fn test_tag() {
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
    }
}
