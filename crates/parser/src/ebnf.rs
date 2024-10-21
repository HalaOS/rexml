//! Xml parsers created by [`nom`] crate.

use nom::{
    branch::alt,
    bytes::{
        complete::take_until,
        streaming::{tag, take_till, take_while, take_while1},
    },
    character::streaming::satisfy,
    combinator::{cond, map, opt, peek},
    error::Error,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use tokens::{XmlPI, XmlVersion};

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

    /// A token represents xml/1.1 `EntityRef`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlEntityRef<'a>(pub &'a str);

    /// A token represents xml/1.1 `CharRef`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlCharRef(pub char);

    /// A token represents xml/1.1 `Reference`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlReference<'a> {
        Entity(&'a str),
        Char(char),
    }

    /// A token represents xml/1.1 `EntityValue`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlEntityValuePart<'a> {
        PEReference(&'a str),
        Entity(&'a str),
        Char(char),
        Unparsed(&'a str),
    }

    /// A token represents xml/1.1 `AttValue`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlAttrValuePart<'a> {
        Entity(&'a str),
        Char(char),
        Unparsed(&'a str),
    }

    /// A token represents xml/1.1 `SystemLiteral`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlSystemId<'a>(pub &'a str);

    /// A token represents xml/1.1 `PubidLiteral`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlPublicId<'a>(pub &'a str);

    /// A token represents xml/1.1 `ExternalID`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlExternalId<'a> {
        System(&'a str),
        Public {
            public_id: &'a str,
            system_id: &'a str,
        },
    }

    /// A token represents xml/1.1 `CharData`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlCharData<'a>(pub &'a str);

    /// A token represents xml/1.1 `Comment`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlComment<'a>(pub &'a str);

    /// A token represents xml/1.1 `PI`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlPI<'a> {
        pub target: &'a str,
        pub data: Option<&'a str>,
    }

    /// A token represents xml/1.1 `PI`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlMisc<'a> {
        PI {
            target: &'a str,
            data: Option<&'a str>,
        },
        Comment(&'a str),
        Space,
    }

    /// A token represents xml/1.1 `CData`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlCData<'a>(pub &'a str);

    /// A token represents xml/1.1 `XmlDecl`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlDecl<'a> {
        pub version: XmlVersion,
        pub encoding: Option<&'a str>,
        pub standalone: Option<bool>,
    }

    /// A token represents xml/1.1 `XmlEncoding`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlEncoding<'a>(pub &'a str);

    /// A token represents xml/1.1 `SDDecl`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlSDDecl(pub bool);

    /// A token represents xml/1.1 `XmlVersion`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlVersion {
        Version1_0,
        Version1_1,
    }

    /// A token represents xml/1.1 `initSubSet`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlInitSubset<'a> {
        PEReference(&'a str),

        Space,
    }

    /// A token represents xml/1.1 `ElementDecl`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlNotationDecl<'a> {
        pub name: XmlName<'a>,
        pub id: XmlNotationId<'a>,
    }

    /// A token represents xml/1.1 `NotationId`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum XmlNotationId<'a> {
        ExternalId(XmlExternalId<'a>),
        PublicId(XmlPublicId<'a>),
    }

    /// A token represents xml/1.1 `NotationId`.
    ///
    /// See [`XML_EBNF1.1`] for more information.
    ///
    /// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct XmlMixed<'a>(pub Option<Vec<&'a str>>);
}

#[allow(unused)]
fn is_restricted_char(c: char) -> bool {
    matches!(
        c, '\u{01}'..='\u{08}' | '\u{0b}'..='\u{0c}' | '\u{0e}'..='\u{1f}'
        | '\u{7f}'..='\u{84}' | '\u{86}'..='\u{9f}'
    )
}

#[allow(unused)]
fn is_char(c: char) -> bool {
    matches!(
        c, '\u{01}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}'
    )
}

#[allow(unused)]
fn is_pubid_char(c: char) -> bool {
    static CHARS: &str = "-'()+,./:=?;!*#@$_%\u{20}'\u{0d}\u{0a}";

    CHARS.contains(c)
        || matches!(
            c, '0'..='9' | 'a'..='z' | 'A'..='Z'
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

    Ok((value, tokens::XmlPEReference(name.0)))
}

/// Parse xml `EntityRef` token.
pub fn xml_entity_ref(value: &str) -> IResult<&str, tokens::XmlEntityRef<'_>> {
    let (value, _) = satisfy(|c| c == '&')(value)?;

    let (value, name) = xml_name(value)?;

    let (value, _) = satisfy(|c| c == ';')(value)?;

    Ok((value, tokens::XmlEntityRef(name.0)))
}

/// Parse xml `CharRef` token.
pub fn xml_char_ref(value: &str) -> IResult<&str, tokens::XmlCharRef> {
    let (value, _) = tag("&#")(value)?;

    let (value, is_hex) = opt(satisfy(|c| c == 'x'))(value)?;

    let (value, digit) = if is_hex.is_some() {
        let (value, digit) = take_while1(|c: char| c.is_ascii_hexdigit())(value)?;

        let digit = match u32::from_str_radix(digit, 16) {
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

        (value, digit)
    } else {
        let (value, digit) = take_while1(|c: char| c.is_ascii_digit())(value)?;

        let digit = match u32::from_str_radix(digit, 10) {
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

        (value, digit)
    };

    let (value, _) = satisfy(|c| c == ';')(value)?;

    Ok((value, tokens::XmlCharRef(digit)))
}

/// Parse xml `Reference` token.
pub fn xml_reference(value: &str) -> IResult<&str, tokens::XmlReference<'_>> {
    alt((
        map(xml_char_ref, |v| tokens::XmlReference::Char(v.0)),
        map(xml_entity_ref, |v| tokens::XmlReference::Entity(v.0)),
    ))(value)
}

/// Parse xml `EntityValue` token.
pub fn xml_entity_value(value: &str) -> IResult<&str, Vec<tokens::XmlEntityValuePart<'_>>> {
    let (mut value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let mut parts = vec![];

    loop {
        let part;
        (value, part) = match opt(alt((
            map(xml_reference, |v| match v {
                tokens::XmlReference::Entity(v) => tokens::XmlEntityValuePart::Entity(v),
                tokens::XmlReference::Char(v) => tokens::XmlEntityValuePart::Char(v),
            }),
            map(xml_pe_reference, |v| {
                tokens::XmlEntityValuePart::PEReference(v.0)
            }),
            map(take_while1(|c| c != '%' && c != '&' && c != start), |v| {
                tokens::XmlEntityValuePart::Unparsed(v)
            }),
        )))(value)
        {
            Ok(r) => r,
            Err(nom::Err::Incomplete(_)) => {
                break;
            }
            Err(err) => return Err(err),
        };

        match part {
            Some(part) => parts.push(part),
            None => break,
        }
    }

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, parts))
}

/// Parse xml `AttrValue` token.
pub fn xml_attr_value(value: &str) -> IResult<&str, Vec<tokens::XmlAttrValuePart<'_>>> {
    let (mut value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let mut parts = vec![];

    loop {
        let part;
        (value, part) = match opt(alt((
            map(xml_reference, |v| match v {
                tokens::XmlReference::Entity(v) => tokens::XmlAttrValuePart::Entity(v),
                tokens::XmlReference::Char(v) => tokens::XmlAttrValuePart::Char(v),
            }),
            map(take_while1(|c| c != '%' && c != '&' && c != start), |v| {
                tokens::XmlAttrValuePart::Unparsed(v)
            }),
        )))(value)
        {
            Ok(r) => r,
            Err(nom::Err::Incomplete(_)) => {
                break;
            }
            Err(err) => return Err(err),
        };

        match part {
            Some(part) => parts.push(part),
            None => break,
        }
    }

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, parts))
}

/// Parse xml `SystemLiteral` token.
pub fn xml_system_id_literal(value: &str) -> IResult<&str, tokens::XmlSystemId<'_>> {
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let (value, token) = map(take_while(|c| c != '%' && c != '&' && c != start), |v| {
        tokens::XmlSystemId(v)
    })(value)?;

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, token))
}

/// Parse xml `PubidLiteral` token.
pub fn xml_public_id_literal(value: &str) -> IResult<&str, tokens::XmlPublicId<'_>> {
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let (value, token) = map(take_while(|c| is_pubid_char(c) && c != start), |v| {
        tokens::XmlPublicId(v)
    })(value)?;

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, token))
}

/// Parse xml `CharData` token.
pub fn xml_char_data(value: &str) -> IResult<&str, tokens::XmlCharData<'_>> {
    let mut input = value;

    loop {
        (input, _) = take_till(|c| c == '<' || c == '&' || c == ']')(input)?;

        if input.chars().next().unwrap() == ']' {
            let cdata_end;
            (input, cdata_end) = peek(opt(tag("]]>")))(input)?;

            if cdata_end.is_none() {
                (_, input) = input.split_at(1);
                continue;
            }
        }

        break;
    }

    let (char_data, value) = value.split_at(value.len() - input.len());

    Ok((value, tokens::XmlCharData(char_data)))
}

/// Parse xml `XmlComment` token.
pub fn xml_comment(value: &str) -> IResult<&str, tokens::XmlComment<'_>> {
    let (value, _) = tag("<!--")(value)?;

    let mut input = value;

    loop {
        (input, _) = take_while(|c| c != '-' && is_char(c))(input)?;

        if input.chars().next().unwrap() == '-' {
            let comment_end;
            (input, comment_end) = peek(opt(tag("--")))(input)?;

            if comment_end.is_none() {
                (_, input) = input.split_at(1);
                continue;
            }
        }

        break;
    }

    let (comment, value) = value.split_at(value.len() - input.len());

    let (value, _) = tag("-->")(value)?;

    Ok((value, tokens::XmlComment(comment)))
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

/// Parse xml `PI` token.
///
/// This function doesn't check the reserved target names 'XML','xml' and so on.
pub fn xml_pi(value: &str) -> IResult<&str, tokens::XmlPI<'_>> {
    let (value, _) = tag("<?")(value)?;

    let (value, target) = xml_name(value)?;

    let (value, space) = opt(xml_space)(value)?;

    let (value, data) = cond(space.is_some(), take_until("?>"))(value)?;

    let (value, _) = tag("?>")(value)?;

    Ok((
        value,
        XmlPI {
            target: target.0,
            data,
        },
    ))
}

/// Parse xml `CData` token.
pub fn xml_cdata(value: &str) -> IResult<&str, tokens::XmlCData<'_>> {
    let (value, _) = tag("<![CDATA[")(value)?;

    let mut input = value;

    loop {
        (input, _) = take_while(|c| c != ']' && is_char(c))(input)?;

        if input.chars().next().unwrap() == ']' {
            let comment_end;
            (input, comment_end) = peek(opt(tag("]]>")))(input)?;

            if comment_end.is_none() {
                (_, input) = input.split_at(1);
                continue;
            }
        }

        break;
    }

    let (cdata, value) = value.split_at(value.len() - input.len());

    let (value, _) = tag("]]>")(value)?;

    Ok((value, tokens::XmlCData(cdata)))
}

/// Parse xml `XmlDecl` token.
pub fn xml_decl(value: &str) -> IResult<&str, tokens::XmlDecl<'_>> {
    let (value, _) = tag("<?xml")(value)?;
    let (value, version) = xml_version(value)?;
    let (value, encoding) = opt(xml_encoding_decl)(value)?;
    let (value, standalone) = opt(xml_standalone_decl)(value)?;
    let (value, _) = opt(xml_space)(value)?;
    let (value, _) = tag("?>")(value)?;

    Ok((
        value,
        tokens::XmlDecl {
            version,
            encoding: encoding.map(|v| v.0),
            standalone: standalone.map(|v| v.0),
        },
    ))
}

/// Parse xml `Version` token.
pub fn xml_version(value: &str) -> IResult<&str, tokens::XmlVersion> {
    let (value, _) = xml_space(value)?;
    let (value, _) = tag("version")(value)?;
    let (value, _) = xml_eq(value)?;
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;
    let (value, version_info) = alt((
        map(tag("1.1"), |_| XmlVersion::Version1_1),
        map(tag("1.0"), |_| XmlVersion::Version1_0),
    ))(value)?;
    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, version_info))
}

/// Parse xml `Eq` token.
pub fn xml_eq(value: &str) -> IResult<&str, ()> {
    let (value, _) = opt(xml_space)(value)?;

    let (value, _) = satisfy(|c| c == '=')(value)?;

    let (value, _) = opt(xml_space)(value)?;

    Ok((value, ()))
}

/// Parse xml `EncName` token.
pub fn xml_encoding_decl(value: &str) -> IResult<&str, tokens::XmlEncoding<'_>> {
    let (value, _) = xml_space(value)?;
    let (value, _) = tag("encoding")(value)?;
    let (value, _) = xml_eq(value)?;
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;
    let (input, _) = satisfy(|c| c.is_ascii_alphabetic())(value)?;
    let (input, _) =
        take_while(|c: char| c == '-' || c == '.' || c == '_' || c.is_alphanumeric())(input)?;

    let (enc_name, value) = value.split_at(value.len() - input.len());

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, tokens::XmlEncoding(enc_name)))
}

/// Parse xml `SDDecl` token.
pub fn xml_standalone_decl(value: &str) -> IResult<&str, tokens::XmlSDDecl> {
    let (value, _) = xml_space(value)?;
    let (value, _) = tag("standalone")(value)?;
    let (value, _) = xml_eq(value)?;
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let (value, sd_decl) = map(alt((tag("yes"), tag("no"))), |v| {
        if v == "yes" {
            tokens::XmlSDDecl(true)
        } else {
            tokens::XmlSDDecl(false)
        }
    })(value)?;

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, sd_decl))
}

/// Parse xml `Misc` token.
pub fn xml_misc(value: &str) -> IResult<&str, tokens::XmlMisc<'_>> {
    alt((
        map(xml_pi, |v| tokens::XmlMisc::PI {
            target: v.target,
            data: v.data,
        }),
        map(xml_comment, |v| tokens::XmlMisc::Comment(v.0)),
        map(xml_space, |_| tokens::XmlMisc::Space),
    ))(value)
}

/// Parse xml `ExternalID` token.
pub fn xml_external_id(value: &str) -> IResult<&str, tokens::XmlExternalId<'_>> {
    let (value, start) = alt((tag("SYSTEM"), tag("PUBLIC")))(value)?;

    let (value, _) = xml_space(value)?;

    let (value, external_id) = if start == "SYSTEM" {
        let (value, system_id) = xml_system_id_literal(value)?;

        (value, tokens::XmlExternalId::System(system_id.0))
    } else {
        let (value, public_id) = xml_public_id_literal(value)?;
        let (value, _) = xml_space(value)?;
        let (value, system_id) = xml_system_id_literal(value)?;

        (
            value,
            tokens::XmlExternalId::Public {
                public_id: public_id.0,
                system_id: system_id.0,
            },
        )
    };

    Ok((value, external_id))
}

/// Parse xml `intSubset` token.
pub fn xml_init_subset(_value: &str) -> IResult<&str, tokens::XmlExternalId<'_>> {
    todo!()
}

/// Parse xml `PublicID` token.
pub fn xml_public_id(value: &str) -> IResult<&str, tokens::XmlPublicId<'_>> {
    let (value, _) = tag("PUBLIC")(value)?;
    let (value, _) = xml_space(value)?;
    xml_public_id_literal(value)
}

/// Parse xml `NotationDecl` token.
pub fn xml_notation_decl(value: &str) -> IResult<&str, tokens::XmlNotationDecl<'_>> {
    let (value, _) = tag("<!NOTATION")(value)?;
    let (value, _) = xml_space(value)?;
    let (value, name) = xml_name(value)?;
    let (value, _) = xml_space(value)?;

    let (value, id) = alt((
        map(xml_external_id, |v| tokens::XmlNotationId::ExternalId(v)),
        map(xml_public_id, |v| tokens::XmlNotationId::PublicId(v)),
    ))(value)?;

    let (value, _) = opt(xml_space)(value)?;

    let (value, _) = satisfy(|c| c == '>')(value)?;

    Ok((value, tokens::XmlNotationDecl { name, id }))
}

/// Parse xml `Mixed` token.
pub fn xml_mixed(value: &str) -> IResult<&str, tokens::XmlMixed<'_>> {
    let (value, _) = satisfy(|c| c == '(')(value)?;

    let (value, _) = opt(xml_space)(value)?;

    let (value, _) = tag("#PCDATA")(value)?;

    let (value, _) = opt(xml_space)(value)?;

    let (value, list) = opt(satisfy(|c| c == '|'))(value)?;

    let (value, types) = if list.is_some() {
        let (value, types) = separated_list0(
            tuple((opt(xml_space), satisfy(|c| c == '|'), opt(xml_space))),
            map(xml_name, |v| v.0),
        )(value)?;

        (value, Some(types))
    } else {
        (value, None)
    };

    let (value, _) = opt(xml_space)(value)?;

    let value = if types.is_none() {
        let (value, _) = satisfy(|c| c == ')')(value)?;
        value
    } else {
        let (value, _) = tag(")*")(value)?;
        value
    };

    Ok((value, tokens::XmlMixed(types)))
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

        assert_eq!(
            xml_name("#hello "),
            Err(nom::Err::Error(Error::new(
                "#hello ",
                nom::error::ErrorKind::Satisfy
            )))
        );
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

    #[test]
    fn entity_ref() {
        let (_, reference) = xml_entity_ref("&hello; ").unwrap();

        assert_eq!(reference, XmlEntityRef("hello"));

        let (_, reference) = xml_entity_ref("&hello:a; ").unwrap();

        assert_eq!(reference, XmlEntityRef("hello:a"));

        xml_entity_ref("&-hello; ").expect_err("name_start_char");

        xml_entity_ref("& hello; ").expect_err("name_start_char");
    }

    #[test]
    fn char_ref() {
        let (_, reference) = xml_char_ref("&#x2122; ").unwrap();

        assert_eq!(reference, XmlCharRef('\u{2122}'));

        let (_, reference) = xml_char_ref("&#169; ").unwrap();

        assert_eq!(reference, XmlCharRef('\u{a9}'));

        assert_eq!(
            xml_char_ref("&#x21222122; "),
            Err(nom::Err::Failure(Error::new(
                "21222122",
                nom::error::ErrorKind::Digit
            )))
        );

        assert_eq!(
            xml_char_ref("&#xd800; "),
            Err(nom::Err::Failure(Error::new(
                "d800",
                nom::error::ErrorKind::Digit
            )))
        );
    }

    #[test]
    fn reference() {
        let (_, reference) = xml_reference("&#x2122; ").unwrap();

        assert_eq!(reference, XmlReference::Char('\u{2122}'));

        let (_, reference) = xml_reference("&hello; ").unwrap();

        assert_eq!(reference, XmlReference::Entity("hello"));
    }

    #[test]
    fn entity_value() {
        let (_, value) = xml_entity_value(r#"'%hello:a; hello &#x2122; world &hello;' "#).unwrap();

        assert_eq!(
            value,
            vec![
                XmlEntityValuePart::PEReference("hello:a"),
                XmlEntityValuePart::Unparsed(" hello "),
                XmlEntityValuePart::Char('™'),
                XmlEntityValuePart::Unparsed(" world "),
                XmlEntityValuePart::Entity("hello")
            ]
        );
    }

    #[test]
    fn attr_value() {
        let (_, value) = xml_attr_value(r#"' hello &#x2122; world &hello;' "#).unwrap();

        assert_eq!(
            value,
            vec![
                XmlAttrValuePart::Unparsed(" hello "),
                XmlAttrValuePart::Char('™'),
                XmlAttrValuePart::Unparsed(" world "),
                XmlAttrValuePart::Entity("hello")
            ]
        );

        xml_attr_value(r#"'%hello:a; hello &#x2122; world &hello;' "#)
            .expect_err("AttValue: unspport PEReference");
    }

    #[test]
    fn system_id() {
        let (_, value) = xml_system_id_literal("'\u{2122}' ").unwrap();

        assert_eq!(value, XmlSystemId("\u{2122}"));

        let (_, value) = xml_system_id_literal(r#"'"hello' "#).unwrap();

        assert_eq!(value, XmlSystemId("\"hello"));

        let (_, value) = xml_system_id_literal(r#""" "#).unwrap();

        assert_eq!(value, XmlSystemId(""));

        let (_, value) = xml_system_id_literal(r#""'hello" "#).unwrap();

        assert_eq!(value, XmlSystemId("'hello"));
    }

    #[test]
    fn public_id() {
        let (_, value) = xml_public_id_literal("'' ").unwrap();

        assert_eq!(value, XmlPublicId(""));

        let (_, value) = xml_public_id_literal(r#""'hello" "#).unwrap();

        assert_eq!(value, XmlPublicId("'hello"));

        assert_eq!(
            xml_public_id_literal("'\u{2122}' "),
            Err(nom::Err::Error(Error::new(
                "\u{2122}' ",
                nom::error::ErrorKind::Satisfy
            )))
        );
    }

    #[test]
    fn chardata() {
        let (_, value) = xml_char_data("hello world&").unwrap();

        assert_eq!(value, XmlCharData("hello world"));

        let (_, value) = xml_char_data("hello world<").unwrap();

        assert_eq!(value, XmlCharData("hello world"));

        let (_, value) = xml_char_data("hello ]> world]]>").unwrap();

        assert_eq!(value, XmlCharData("hello ]> world"));
    }

    #[test]
    fn comment() {
        let (_, value) = xml_comment("<!-- hello - world& --> ").unwrap();

        assert_eq!(value, XmlComment(" hello - world& "));

        let (_, value) = xml_comment("<!--hello - <<<!>> world&--> ").unwrap();

        assert_eq!(value, XmlComment("hello - <<<!>> world&"));

        assert_eq!(
            xml_comment("<!-- hello -- world& --> "),
            Err(nom::Err::Error(Error::new(
                "-- world& --> ",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn pi() {
        let (_, value) = xml_pi("<?xml-stylesheet?> ").unwrap();

        assert_eq!(
            value,
            XmlPI {
                target: "xml-stylesheet",
                data: None
            }
        );

        let (_, value) = xml_pi("<?xml-stylesheet ?> ").unwrap();

        assert_eq!(
            value,
            XmlPI {
                target: "xml-stylesheet",
                data: Some("")
            }
        );

        let (_, value) = xml_pi("<?xml-stylesheet   hello<>world   ?> ").unwrap();

        assert_eq!(
            value,
            XmlPI {
                target: "xml-stylesheet",
                data: Some("hello<>world   ")
            }
        );
    }

    #[test]
    fn cdata() {
        let (_, value) = xml_cdata("<![CDATA[]]> ").unwrap();

        assert_eq!(value, XmlCData(""));

        let (_, value) = xml_cdata("<![CDATA[ he<![CDATA[]]llo]]> ").unwrap();

        assert_eq!(value, XmlCData(" he<![CDATA[]]llo"));
    }

    #[test]
    fn xmldecl() {
        let (_, value) = xml_decl(r#"<?xml version="1.0" encoding="utf-8"?> "#).unwrap();

        assert_eq!(
            value,
            XmlDecl {
                version: XmlVersion::Version1_0,
                encoding: Some("utf-8"),
                standalone: None
            }
        );

        let (_, value) =
            xml_decl(r#"<?xml version="1.1" encoding="utf-8" standalone="yes"       ?> "#).unwrap();

        assert_eq!(
            value,
            XmlDecl {
                version: XmlVersion::Version1_1,
                encoding: Some("utf-8"),
                standalone: Some(true)
            }
        );
    }

    #[test]
    fn external_id() {
        let (_, id) =
            xml_external_id(r#"SYSTEM "http://www.textuality.com/boilerplate/OpenHatch.xml" "#)
                .unwrap();

        assert_eq!(
            id,
            XmlExternalId::System("http://www.textuality.com/boilerplate/OpenHatch.xml")
        );

        let (_, id) = xml_external_id(
            r#"PUBLIC "-//Textuality//TEXT Standard open-hatch boilerplate//EN" 
                "http://www.textuality.com/boilerplate/OpenHatch.xml" "#,
        )
        .unwrap();

        assert_eq!(
            id,
            XmlExternalId::Public {
                public_id: "-//Textuality//TEXT Standard open-hatch boilerplate//EN",
                system_id: "http://www.textuality.com/boilerplate/OpenHatch.xml"
            }
        );
    }

    #[test]
    fn mixed() {
        let (_, mixed) = xml_mixed(r#"(#PCDATA) "#).unwrap();

        assert_eq!(mixed, XmlMixed(None));

        assert_eq!(xml_mixed(r#"(#PCDATA)* "#), Ok(("* ", XmlMixed(None))));

        let (_, mixed) = xml_mixed(r#"(#PCDATA|a|ul|b|i|em)* "#).unwrap();

        assert_eq!(mixed, XmlMixed(Some(vec!["a", "ul", "b", "i", "em"])));

        // let (_, mixed) =
        //     xml_mixed(r#"(#PCDATA | %font; | %phrase; | %special; | %form;)* "#).unwrap();

        // assert_eq!(
        //     mixed,
        //     XmlMixed(Some(vec!["%font;", "%phrase;", "%special;", "%form;",]))
        // );
    }
}
