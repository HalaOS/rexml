//! Xml parsers created by [`nom`] crate.

use nom::{
    bytes::streaming::{take_while, take_while1},
    character::streaming::satisfy,
    combinator::opt,
    IResult,
};
use tokens::XmlPEReference;

/// Parsed xml tokens.
pub mod tokens {
    /// A token represents xml/1.1 `Name`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlName<'a>(pub &'a str);

    /// A token represents xml/1.1 `NmToken`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlNmToken<'a>(pub &'a str);

    /// A token represents xml/1.1 `PEReference`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlPEReference<'a>(pub &'a str);
}

#[allow(unused)]
fn is_restricted_char(c: char) -> bool {
    matches!(
        c, '\u{01}'..='\u{08}' | '\u{0b}'..='\u{0c}' | '\u{0e}'..='\u{1f}'
        | '\u{7f}'..='\u{84}' | '\u{86}'..='\u{9f}'
    )
}

fn is_name_char(c: char) -> bool {
    is_name_start_char(c)
        || c == '-'
        || c == '.'
        || matches!(
            c, '0'..='9' | '\u{b7}' | '\u{0300}'..='\u{036f}'|
            '\u{203f}'..='\u{2040}'
        )
}
fn is_name_start_char(c: char) -> bool {
    c == ':'
        || c == '_'
        || matches!(
            c, 'A'..='Z' | 'a'..='z' | '\u{c0}'..='\u{d6}'
            | '\u{d8}'..='\u{f6}' | '\u{f8}'..='\u{2ff}'
            | '\u{370}'..='\u{37d}' | '\u{37f}'..='\u{1fff}'
            | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}'
            | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}'
            | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}'
            | '\u{10000}'..='\u{effff}'
        )
}

/// Parse xml `S` token.
pub fn xml_space(value: &str) -> IResult<&str, ()> {
    let (value, _) = take_while1(|c: char| c.is_ascii_whitespace())(value)?;

    Ok((value, ()))
}

/// Parse xml `PEReference` token.
pub fn xml_pe_reference(value: &str) -> IResult<&str, tokens::XmlPEReference<'_>> {
    let (value, _) = satisfy(|c| c == '%')(value)?;

    let (value, name) = xml_name(value)?;

    let (value, _) = satisfy(|c| c == ';')(value)?;

    Ok((value, XmlPEReference(name.0)))
}

/// Parse xml `NmToken` token.
pub fn xml_nm_token(value: &str) -> IResult<&str, tokens::XmlNmToken<'_>> {
    let (value, body) = take_while(is_name_char)(value)?;

    Ok((value, tokens::XmlNmToken(body)))
}

/// Parse xml `NmTokens` token.
pub fn xml_nm_tokens(value: &str) -> IResult<&str, Vec<tokens::XmlNmToken<'_>>> {
    let (mut value, name) = xml_nm_token(value)?;

    let mut names = vec![name];

    loop {
        let (input, ws) = match opt(xml_space)(value) {
            Ok(r) => r,
            Err(nom::Err::Incomplete(_)) => {
                break;
            }
            Err(err) => return Err(err),
        };

        if ws.is_none() || input.is_empty() {
            break;
        }

        let (input, name) = match xml_nm_token(input) {
            Ok(r) => r,
            Err(r) => {
                return Err(r);
            }
        };

        names.push(name);

        value = input;
    }

    Ok((value, names))
}

/// Parse xml `Name` token.
pub fn xml_name(value: &str) -> IResult<&str, tokens::XmlName<'_>> {
    let (input, _) = satisfy(is_name_start_char)(value)?;

    let (input, body) = take_while(is_name_char)(input)?;

    let (name, _) = value.split_at(1 + body.len());

    Ok((input, tokens::XmlName(name)))
}

/// Parse xml `Names` token.
pub fn xml_names(value: &str) -> IResult<&str, Vec<tokens::XmlName<'_>>> {
    let (mut value, name) = xml_name(value)?;

    let mut names = vec![name];

    loop {
        let (input, ws) = match opt(xml_space)(value) {
            Ok(r) => r,
            Err(nom::Err::Incomplete(_)) => {
                break;
            }
            Err(err) => return Err(err),
        };

        if ws.is_none() || input.is_empty() {
            break;
        }

        let (input, name) = match xml_name(input) {
            Ok(r) => r,
            Err(r) => {
                return Err(r);
            }
        };

        names.push(name);

        value = input;
    }

    Ok((value, names))
}

#[cfg(test)]
mod tests {
    use tokens::*;

    use super::*;

    #[test]
    fn name() {
        let (_, name) = xml_name("hello:a ").unwrap();

        assert_eq!(name, XmlName("hello:a"));

        let (_, name) = xml_name("hello ").unwrap();

        assert_eq!(name, XmlName("hello"));

        let (_, name) = xml_name(":hello ").unwrap();

        assert_eq!(name, XmlName(":hello"));

        xml_name(".hello ").expect_err("name_start_char");
        xml_name("-hello ").expect_err("name_start_char");
        xml_name("0hello ").expect_err("name_start_char");
    }

    #[test]
    fn names() {
        let (_, names) = xml_names("hello:a hello:b hello :b ").unwrap();

        assert_eq!(
            names,
            vec![
                XmlName("hello:a"),
                XmlName("hello:b"),
                XmlName("hello"),
                XmlName(":b")
            ]
        );
    }

    #[test]
    fn nm_tokens() {
        let (_, names) = xml_nm_tokens(".hello:a -hello:b 9hello :b ").unwrap();

        assert_eq!(
            names,
            vec![
                XmlNmToken(".hello:a"),
                XmlNmToken("-hello:b"),
                XmlNmToken("9hello"),
                XmlNmToken(":b")
            ]
        );
    }

    #[test]
    fn pe_reference() {
        let (_, reference) = xml_pe_reference("%hello; ").unwrap();

        assert_eq!(reference, XmlPEReference("hello"));

        let (_, reference) = xml_pe_reference("%hello:a; ").unwrap();

        assert_eq!(reference, XmlPEReference("hello:a"));

        xml_pe_reference("%-hello; ").expect_err("name_start_char");
    }
}
