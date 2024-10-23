//! Xml parsers created by [`nom`] crate.

use nom::{
    branch::alt,
    bytes::streaming::{tag, take_till1, take_while, take_while1},
    character::streaming::satisfy,
    combinator::map,
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::symbols::{
    XmlEntityValuePart, XmlName, XmlNmToken, XmlPEReference, XmlPubidLiteral, XmlReference,
    XmlWhiteSpace,
};

/// [#x1-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF],
/// any Unicode character, excluding the surrogate blocks, FFFE, and FFFF.
pub fn is_char(c: char) -> bool {
    matches!(c, '\u{1}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}')
}

/// #x1-#x8] | [#xB-#xC] | [#xE-#x1F] | [#x7F-#x84] | [#x86-#x9F]
pub fn is_restricted_char(c: char) -> bool {
    matches!(c, '\u{1}'..='\u{8}' | '\u{b}'..='\u{c}' | '\u{e}'..='\u{1f}' | '\u{7f}'..='\u{84}' | '\u{86}'..='\u{9f}')
}

/// (#x20 | #x9 | #xD | #xA)+
pub fn is_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\x0C' | '\r' | ' ')
}

/// NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub fn is_name_char(c: char) -> bool {
    is_name_start_char(c)
        || matches!(
            c, '-' | '.' | '0'..='9' | '\u{b7}' | '\u{0300}'..='\u{036f}'|
            '\u{203f}'..='\u{2040}'
        )
}

/// ":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] |
/// [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] |
/// [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] |
/// [#x10000-#xEFFFF]
pub fn is_name_start_char(c: char) -> bool {
    matches!(
        c, ':' | '_' | 'A'..='Z' | 'a'..='z' | '\u{c0}'..='\u{d6}'
        | '\u{d8}'..='\u{f6}' | '\u{f8}'..='\u{2ff}'
        | '\u{370}'..='\u{37d}' | '\u{37f}'..='\u{1fff}'
        | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}'
        | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}'
        | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}'
        | '\u{10000}'..='\u{effff}'
    )
}

/// #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
pub fn is_pubid_char(c: char) -> bool {
    matches!(c,
        '-' | '\'' | '(' | ')' | '+' | ',' | '.' | '/' | ':' | '=' |
         '?' | ';' | '!' | '*' | '#' | '@' | '$' | '_' | '%' |
        '\u{20}' | '\u{d}' | '\u{a}' | 'A'..='Z' | 'a'..='z' | '0'..='9'
    )
}

/// Parse `White space` symbol.
pub fn xml_ws(value: &str) -> IResult<&str, XmlWhiteSpace<'_>> {
    map(take_while(is_whitespace), |v| XmlWhiteSpace(v))(value)
}

/// Parse `Name` symbol.
pub fn xml_name(value: &str) -> IResult<&str, XmlName<'_>> {
    map(
        pair(satisfy(is_name_start_char), take_while(is_name_char)),
        |(_, body): (char, &str)| {
            let (name, _) = value.split_at(body.len() + 1);

            XmlName(name)
        },
    )(value)
}

/// Parse `Names` symbol.
pub fn xml_names(value: &str) -> IResult<&str, Vec<XmlName<'_>>> {
    separated_list1(satisfy(|c| c == '\u{20}'), xml_name)(value)
}

/// Parse `NmToken` symbol.
pub fn xml_nmtoken(value: &str) -> IResult<&str, XmlNmToken<'_>> {
    map(take_while1(is_name_char), |v| XmlNmToken(v))(value)
}

/// Parse `NmToken` symbol.
pub fn xml_nmtokens(value: &str) -> IResult<&str, Vec<XmlNmToken<'_>>> {
    separated_list1(satisfy(|c| c == '\u{20}'), xml_nmtoken)(value)
}

/// Parse `PubidLiteral` symbol.
pub fn xml_pubid_lit(value: &str) -> IResult<&str, XmlPubidLiteral<'_>> {
    map(
        alt((
            delimited(
                satisfy(|c| c == '"'),
                take_while(is_pubid_char),
                satisfy(|c| c == '"'),
            ),
            delimited(
                satisfy(|c| c == '\''),
                take_while(|c| c != '\'' && is_pubid_char(c)),
                satisfy(|c| c == '\''),
            ),
        )),
        |v| XmlPubidLiteral(v),
    )(value)
}

fn xml_char_ref(value: &str) -> IResult<&str, char> {
    let (value, (hex, digit)) = alt((
        map(
            tuple((
                tag("&#"),
                satisfy(|c| c == 'x'),
                take_while1(|c: char| c.is_ascii_hexdigit()),
                satisfy(|c| c == ';'),
            )),
            |(_, _, v, _)| (true, v),
        ),
        map(
            tuple((
                tag("&#"),
                take_while1(|c: char| c.is_ascii_digit()),
                satisfy(|c| c == ';'),
            )),
            |(_, v, _)| (false, v),
        ),
    ))(value)?;

    let digit = match u32::from_str_radix(digit, if hex { 16 } else { 10 }) {
        Ok(v) => char::from_u32(v).ok_or(nom::Err::Failure(Error::new(
            digit,
            nom::error::ErrorKind::Digit,
        )))?,
        Err(_) => {
            return Err(nom::Err::Failure(Error::new(
                digit,
                nom::error::ErrorKind::Digit,
            )))
        }
    };

    Ok((value, digit))
}

pub fn xml_entity_ref(value: &str) -> IResult<&str, &str> {
    let (value, _) = satisfy(|c| c == '&')(value)?;

    let (value, name) = xml_name(value)?;

    let (value, _) = satisfy(|c| c == ';')(value)?;

    Ok((value, name.0))
}

/// Parse `Reference` symbol.
pub fn xml_ref(value: &str) -> IResult<&str, XmlReference<'_>> {
    alt((
        map(xml_char_ref, |v| XmlReference::Char(v)),
        map(xml_entity_ref, |v| XmlReference::Entity(v)),
    ))(value)
}

/// Parse `PEReference` symbol.
pub fn xml_peref(value: &str) -> IResult<&str, XmlPEReference<'_>> {
    let (value, _) = satisfy(|c| c == '%')(value)?;

    let (value, name) = xml_name(value)?;

    let (value, _) = satisfy(|c| c == ';')(value)?;

    Ok((value, XmlPEReference(name.0)))
}

fn xml_entity_value_parts(quote: char) -> impl Fn(&str) -> IResult<&str, XmlEntityValuePart<'_>> {
    move |value| {
        alt((
            map(take_till1(|c| c == quote || c == '&' || c == '%'), |v| {
                XmlEntityValuePart::Literal(v)
            }),
            map(xml_ref, |v| XmlEntityValuePart::Reference(v)),
            map(xml_peref, |v| XmlEntityValuePart::PEReference(v.0)),
        ))(value)
    }
}

/// Parse `EntityValue` symbol.
pub fn xml_entity_value(value: &str) -> IResult<&str, Vec<XmlEntityValuePart<'_>>> {
    let (value, (_, parts, _)) = alt((
        tuple((tag("'"), many0(xml_entity_value_parts('\'')), tag("'"))),
        tuple((tag("\""), many0(xml_entity_value_parts('"')), tag("\""))),
    ))(value)?;

    Ok((value, parts))
}
#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn name() {
        let (_, symbol) = xml_name(":hello:world ").unwrap();

        assert_eq!(symbol, XmlName(":hello:world"));

        assert_eq!(
            xml_name(".hello:world "),
            Err(nom::Err::Error(nom::error::Error::new(
                ".hello:world ",
                ErrorKind::Satisfy
            )))
        );

        assert_eq!(
            xml_name("-hello:world "),
            Err(nom::Err::Error(nom::error::Error::new(
                "-hello:world ",
                ErrorKind::Satisfy
            )))
        );

        assert_eq!(
            xml_name("9hello:world "),
            Err(nom::Err::Error(nom::error::Error::new(
                "9hello:world ",
                ErrorKind::Satisfy
            )))
        );
    }

    #[test]
    fn names() {
        let (_, symbol) = xml_names(":hello:world :hello:world :hello:world  ").unwrap();

        assert_eq!(
            symbol,
            vec![
                XmlName(":hello:world"),
                XmlName(":hello:world"),
                XmlName(":hello:world")
            ]
        );
    }

    #[test]
    fn nmtoken() {
        let (_, symbol) = xml_nmtoken("9hello:world ").unwrap();

        assert_eq!(symbol, XmlNmToken("9hello:world"));
    }

    #[test]
    fn nmtokens() {
        let (_, symbol) = xml_nmtokens("9hello:world 7hello:world 8hello:world>").unwrap();

        assert_eq!(
            symbol,
            vec![
                XmlNmToken("9hello:world"),
                XmlNmToken("7hello:world"),
                XmlNmToken("8hello:world")
            ]
        );
    }

    #[test]
    fn pubid_lit() {
        let (_, symbol) = xml_pubid_lit(r#""-/he'llo" "#).unwrap();

        assert_eq!(symbol, XmlPubidLiteral(r#"-/he'llo"#));

        let (_, symbol) = xml_pubid_lit(r#"'-/hello' "#).unwrap();

        assert_eq!(symbol, XmlPubidLiteral(r#"-/hello"#));
    }

    #[test]
    fn reference() {
        let (_, reference) = xml_ref("&#x2122; ").unwrap();

        assert_eq!(reference, XmlReference::Char('\u{2122}'));

        let (_, reference) = xml_ref("&#10; ").unwrap();

        assert_eq!(reference, XmlReference::Char('\u{a}'));

        let (_, reference) = xml_ref("&hello; ").unwrap();

        assert_eq!(reference, XmlReference::Entity("hello"));
    }

    #[test]
    fn entity_value() {
        let (_, value) = xml_entity_value(r#"'%hello:a; hello &#x2122; world &hello;' "#).unwrap();

        assert_eq!(
            value,
            vec![
                XmlEntityValuePart::PEReference("hello:a"),
                XmlEntityValuePart::Literal(" hello "),
                XmlEntityValuePart::Reference(XmlReference::Char('™')),
                XmlEntityValuePart::Literal(" world "),
                XmlEntityValuePart::Reference(XmlReference::Entity("hello"))
            ]
        );
    }
}
