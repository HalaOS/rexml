//! Xml parsers that use the [`&str`] as the input data.

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::space0,
    combinator::opt,
    IResult,
};

use crate::{character::nc_name, sax::SaxHandler, Error};

/// Parse the input [`&str`] into an xml document.
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
fn parse_pi(value: &str) -> IResult<&str, (&str, Option<&str>), Error<'_>> {
    let (value, _) = tag("<?")(value)?;

    let (value, target) = nc_name(value)?;

    let (value, _) = space0(value)?;

    let (value, data) = opt(is_not("?>"))(value)?;

    let (value, _) = tag("?>")(value)?;

    Ok((value, (target, data)))
}

#[allow(unused)]
fn parse_element_start(value: &str) -> IResult<&str, (&str, Vec<(&str, &str)>), Error<'_>> {
    let (value, _) = tag("<")(value)?;

    let (value, qname) = nc_name(value)?;

    let (value, _) = alt((tag("/>"), tag(">")))(value)?;

    todo!()
}

#[allow(unused)]
fn parse_attr(value: &str) -> IResult<&str, (&str, &str), Error<'_>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{parse_element_start, parse_pi};

    #[test]
    fn test_pi() {
        let (_, (target, data)) = parse_pi(r#"<?xml version="1.0"?>"#).unwrap();

        assert_eq!(target, "xml");
        assert_eq!(data, Some(r#"version="1.0""#));

        let (_, (target, data)) = parse_pi(r#"<?xml-stylesheet ?>"#).unwrap();

        assert_eq!(target, "xml-stylesheet");
        assert_eq!(data, None);
    }

    #[test]
    fn test_element() {
        let (_, (qname, _attrs)) = parse_element_start(r#"<xml version="1.0">"#).unwrap();

        assert_eq!(qname, "xml");
    }
}
