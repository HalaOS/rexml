use nom::{
    bytes::complete::{is_not, tag},
    character::complete::space0,
    combinator::opt,
    IResult,
};

use crate::{character::nc_name, sax::SaxHandler, Error};

/// Parse a xml document with [`SaxHandler`] backend.
#[allow(unused)]
pub fn parse_document<'a, H>(value: &'a str, handler: &mut H) -> IResult<&'a str, (), Error<'a>>
where
    H: SaxHandler<'a>,
{
    handler.start_document().map_err(|err| err.into())?;

    let (value, pi) = opt(parse_pi)(value)?;

    if let Some((target, data)) = pi {
        handler
            .processing_instruction(target.into(), data.map(|data| data.into()))
            .map_err(|err| err.into())?;
    }

    handler.end_document().map_err(|err| err.into())?;

    Ok((value, ()))
}

/// Parse a xml `processing instruction`.
pub fn parse_pi(value: &str) -> IResult<&str, (&str, Option<&str>), Error<'_>> {
    let (value, _) = tag("<?")(value)?;

    let (value, target) = nc_name(value)?;

    let (value, _) = space0(value)?;

    let (value, data) = opt(is_not("?>"))(value)?;

    let (value, _) = tag("?>")(value)?;

    Ok((value, (target, data)))
}

#[cfg(test)]
mod tests {
    use super::parse_pi;

    #[test]
    fn test_pi() {
        parse_pi(r#"<?xml version="1.0"?>"#).unwrap();
    }
}
