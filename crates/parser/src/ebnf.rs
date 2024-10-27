//! Xml parsers created by [`nom`] crate.

use nom::{
    branch::alt,
    bytes::streaming::{tag, take_till1, take_until, take_while, take_while1},
    character::streaming::satisfy,
    combinator::{cond, map, opt, peek},
    error::Error,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::symbols::{
    XmlAttDef, XmlAttListDecl, XmlAttType, XmlAttValuePart, XmlCData, XmlCP, XmlChildren,
    XmlComment, XmlContentSpec, XmlDecl, XmlDeclName, XmlDeclSep, XmlDefaultDecl, XmlDocTypeDecl,
    XmlElementDecl, XmlEncoding, XmlEntityDecl, XmlEntityDef, XmlEntityValuePart, XmlEnumType,
    XmlExternalId, XmlMarkupDecl, XmlMisc, XmlMixed, XmlNDataDecl, XmlName, XmlNmToken,
    XmlNotationDecl, XmlNotationId, XmlPEDef, XmlPEReference, XmlPI, XmlPubidLiteral, XmlPublicId,
    XmlReference, XmlRepeat, XmlSDDecl, XmlSystemLiteral, XmlTokenizedType, XmlVersion,
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
    map(take_while1(is_whitespace), |v| XmlWhiteSpace(v))(value)
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

/// Parse `SystemLiteral` symbol.
pub fn xml_system_lit(value: &str) -> IResult<&str, XmlSystemLiteral<'_>> {
    map(
        alt((
            delimited(
                satisfy(|c| c == '"'),
                take_while(|c| c != '"'),
                satisfy(|c| c == '"'),
            ),
            delimited(
                satisfy(|c| c == '\''),
                take_while(|c| c != '\''),
                satisfy(|c| c == '\''),
            ),
        )),
        |v| XmlSystemLiteral(v),
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
            map(xml_ref, |v| match v {
                XmlReference::Char(v) => XmlEntityValuePart::CharRef(v),
                XmlReference::Entity(v) => XmlEntityValuePart::EntityRef(v),
            }),
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

fn xml_attr_value_parts(quote: char) -> impl Fn(&str) -> IResult<&str, XmlAttValuePart<'_>> {
    move |value| {
        alt((
            map(take_till1(|c| c == quote || c == '&' || c == '%'), |v| {
                XmlAttValuePart::Literal(v)
            }),
            map(xml_ref, |v| match v {
                XmlReference::Char(v) => XmlAttValuePart::CharRef(v),
                XmlReference::Entity(v) => XmlAttValuePart::EntityRef(v),
            }),
        ))(value)
    }
}

/// Parse `AttrValue` symbol.
pub fn xml_att_value(value: &str) -> IResult<&str, Vec<XmlAttValuePart<'_>>> {
    let (value, (_, parts, _)) = alt((
        tuple((tag("'"), many0(xml_attr_value_parts('\'')), tag("'"))),
        tuple((tag("\""), many0(xml_attr_value_parts('"')), tag("\""))),
    ))(value)?;

    Ok((value, parts))
}

/// Parse `Comment` symbol.
pub fn xml_comment(value: &str) -> IResult<&str, XmlComment<'_>> {
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

    Ok((value, XmlComment(comment)))
}

/// Parse xml `PI` token.
///
/// This function doesn't check the reserved target names 'XML','xml' and so on.
pub fn xml_pi(value: &str) -> IResult<&str, XmlPI<'_>> {
    let (value, _) = tag("<?")(value)?;

    let (value, target) = xml_name(value)?;

    let (value, space) = opt(xml_ws)(value)?;

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
pub fn xml_cdata(value: &str) -> IResult<&str, XmlCData<'_>> {
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

    Ok((value, XmlCData(cdata)))
}

/// Parse xml `XmlDecl` token.
pub fn xml_decl(value: &str) -> IResult<&str, XmlDecl<'_>> {
    let (value, _) = tag("<?xml")(value)?;
    let (value, version) = xml_version(value)?;
    let (value, encoding) = opt(xml_encoding_decl)(value)?;
    let (value, standalone) = opt(xml_standalone_decl)(value)?;
    let (value, _) = opt(xml_ws)(value)?;
    let (value, _) = tag("?>")(value)?;

    Ok((
        value,
        XmlDecl {
            version,
            encoding: encoding.map(|v| v.0),
            standalone: standalone.map(|v| v.0),
        },
    ))
}

/// Parse xml `Version` token.
pub fn xml_version(value: &str) -> IResult<&str, XmlVersion> {
    let (value, _) = xml_ws(value)?;
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
    let (value, _) = opt(xml_ws)(value)?;

    let (value, _) = satisfy(|c| c == '=')(value)?;

    let (value, _) = opt(xml_ws)(value)?;

    Ok((value, ()))
}

/// Parse xml `EncName` token.
pub fn xml_encoding_decl(value: &str) -> IResult<&str, XmlEncoding<'_>> {
    let (value, _) = xml_ws(value)?;
    let (value, _) = tag("encoding")(value)?;
    let (value, _) = xml_eq(value)?;
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;
    let (input, _) = satisfy(|c| c.is_ascii_alphabetic())(value)?;
    let (input, _) =
        take_while(|c: char| c == '-' || c == '.' || c == '_' || c.is_alphanumeric())(input)?;

    let (enc_name, value) = value.split_at(value.len() - input.len());

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, XmlEncoding(enc_name)))
}

/// Parse xml `SDDecl` token.
pub fn xml_standalone_decl(value: &str) -> IResult<&str, XmlSDDecl> {
    let (value, _) = xml_ws(value)?;
    let (value, _) = tag("standalone")(value)?;
    let (value, _) = xml_eq(value)?;
    let (value, start) = satisfy(|c| c == '"' || c == '\'')(value)?;

    let (value, sd_decl) = map(alt((tag("yes"), tag("no"))), |v| {
        if v == "yes" {
            XmlSDDecl(true)
        } else {
            XmlSDDecl(false)
        }
    })(value)?;

    let (value, _) = satisfy(|c| c == start)(value)?;

    Ok((value, sd_decl))
}

/// Parse xml `Misc` token.
pub fn xml_misc(value: &str) -> IResult<&str, XmlMisc<'_>> {
    alt((
        map(xml_pi, |v| XmlMisc::PI {
            target: v.target,
            data: v.data,
        }),
        map(xml_comment, |v| XmlMisc::Comment(v.0)),
        map(xml_ws, |v| XmlMisc::Space(v.0)),
    ))(value)
}

/// Parse xml `ExternalID` token.
pub fn xml_external_id(value: &str) -> IResult<&str, XmlExternalId<'_>> {
    let (value, start) = alt((tag("SYSTEM"), tag("PUBLIC")))(value)?;

    let (value, _) = xml_ws(value)?;

    let (value, external_id) = if start == "SYSTEM" {
        let (value, system_id) = xml_system_lit(value)?;

        (value, XmlExternalId::System(system_id.0))
    } else {
        let (value, public_id) = xml_pubid_lit(value)?;
        let (value, _) = xml_ws(value)?;
        let (value, system_id) = xml_system_lit(value)?;

        (
            value,
            XmlExternalId::Public {
                public_id: public_id.0,
                system_id: system_id.0,
            },
        )
    };

    Ok((value, external_id))
}

/// Parse xml `PublicID` token.
pub fn xml_public_id(value: &str) -> IResult<&str, XmlPublicId<'_>> {
    let (value, _) = tag("PUBLIC")(value)?;
    let (value, _) = xml_ws(value)?;

    map(xml_pubid_lit, |v| XmlPublicId(v.0))(value)
}

/// Parse xml `NDataDecl` token.
pub fn xml_ndata_decl(value: &str) -> IResult<&str, XmlNDataDecl<'_>> {
    map(
        tuple((xml_ws, tag("NDATA"), xml_ws, xml_name)),
        |(_, _, _, name)| XmlNDataDecl(name.0),
    )(value)
}

/// Parse xml `DeclSep` token.
pub fn xml_decl_sep(value: &str) -> IResult<&str, XmlDeclSep<'_>> {
    alt((
        map(xml_ws, |v| XmlDeclSep::Space(v.0)),
        map(xml_peref, |v| XmlDeclSep::PEReference(v.0)),
    ))(value)
}

/// Parse xml `NotationDecl` token.
pub fn xml_notation_decl(value: &str) -> IResult<&str, XmlNotationDecl<'_>> {
    let (value, _) = tag("<!NOTATION")(value)?;
    let (value, _) = xml_ws(value)?;
    let (value, name) = xml_name(value)?;
    let (value, _) = xml_ws(value)?;

    let (value, id) = alt((
        map(xml_external_id, |v| match v {
            XmlExternalId::System(v) => XmlNotationId::System(v),
            XmlExternalId::Public {
                public_id,
                system_id,
            } => XmlNotationId::PublicSystem {
                public_id,
                system_id,
            },
        }),
        map(xml_public_id, |v| XmlNotationId::Public(v.0)),
    ))(value)?;

    let (value, _) = opt(xml_ws)(value)?;

    let (value, _) = satisfy(|c| c == '>')(value)?;

    Ok((value, XmlNotationDecl { name: name.0, id }))
}

/// Parse xml `NotationDecl` token.
pub fn xml_entity_decl(value: &str) -> IResult<&str, XmlEntityDecl<'_>> {
    let (value, _) = tuple((tag("<!ENTITY"), xml_ws))(value)?;

    let (mut value, is_pe) = opt(satisfy(|c| c == '%'))(value)?;

    if is_pe.is_some() {
        (value, _) = xml_ws(value)?;
    }

    let (value, name) = xml_name(value)?;

    let (value, _) = xml_ws(value)?;

    let (value, decl) = if is_pe.is_none() {
        map(
            alt((
                map(xml_entity_value, |v| XmlEntityDef::Value(v)),
                map(
                    tuple((xml_external_id, opt(xml_ndata_decl))),
                    |(id, ndata_decl)| XmlEntityDef::External {
                        id,
                        ndata_decl: ndata_decl.map(|v| v.0),
                    },
                ),
            )),
            |v| XmlEntityDecl::GEDecl {
                name: name.0,
                def: v,
            },
        )(value)?
    } else {
        map(
            alt((
                map(xml_entity_value, |v| XmlPEDef::Value(v)),
                map(xml_external_id, |id| XmlPEDef::External(id)),
            )),
            |v| XmlEntityDecl::PEDecl {
                name: name.0,
                def: v,
            },
        )(value)?
    };

    let (value, _) = opt(xml_ws)(value)?;

    let (value, _) = satisfy(|c| c == '>')(value)?;

    Ok((value, decl))
}

/// Parse xml `EnumeratedType` token.
pub fn xml_enum_type(value: &str) -> IResult<&str, XmlEnumType<'_>> {
    alt((
        map(
            tuple((
                tag("NOTATION"),
                xml_ws,
                delimited(
                    tag("("),
                    separated_list1(tag("|"), tuple((opt(xml_ws), xml_name, opt(xml_ws)))),
                    tag(")"),
                ),
            )),
            |(_, _, v)| XmlEnumType::Notation(v.into_iter().map(|(_, v, _)| v.0).collect()),
        ),
        map(
            delimited(
                tag("("),
                separated_list1(tag("|"), tuple((opt(xml_ws), xml_nmtoken, opt(xml_ws)))),
                tag(")"),
            ),
            |v| XmlEnumType::NmToken(v.into_iter().map(|(_, v, _)| v.0).collect()),
        ),
    ))(value)
}

/// Parse xml `TokenizedType` token.
pub fn xml_tokenized_type(value: &str) -> IResult<&str, XmlTokenizedType> {
    alt((
        map(tag("IDREFS"), |_| XmlTokenizedType::IdRefs),
        map(tag("IDREF"), |_| XmlTokenizedType::IdRef),
        map(tag("ID"), |_| XmlTokenizedType::Id),
        map(tag("ENTITY"), |_| XmlTokenizedType::Entity),
        map(tag("ENTITIES"), |_| XmlTokenizedType::Entities),
        map(tag("NMTOKENS"), |_| XmlTokenizedType::NmTokens),
        map(tag("NMTOKEN"), |_| XmlTokenizedType::NmToken),
    ))(value)
}

/// Parse xml `TokenizedType` token.
pub fn xml_att_type(value: &str) -> IResult<&str, XmlAttType<'_>> {
    alt((
        map(tag("CDATA"), |_| XmlAttType::String),
        map(xml_enum_type, |v| match v {
            XmlEnumType::Notation(v) => XmlAttType::Notation(v),
            XmlEnumType::NmToken(v) => XmlAttType::NmToken(v),
        }),
        map(xml_tokenized_type, |v| XmlAttType::Tokenized(v)),
    ))(value)
}

/// Parse xml `DefaultDecl` token.
pub fn xml_default_decl(value: &str) -> IResult<&str, XmlDefaultDecl<'_>> {
    alt((
        map(tag("#REQUIRED"), |_| XmlDefaultDecl::Required),
        map(tag("#IMPLIED"), |_| XmlDefaultDecl::Implied),
        map(
            tuple((opt(tuple((tag("#FIXED"), xml_ws))), xml_att_value)),
            |(_, v)| XmlDefaultDecl::Fixed(v),
        ),
    ))(value)
}

/// Parse xml `DefaultDecl` token.
pub fn xml_att_list_decl(value: &str) -> IResult<&str, XmlAttListDecl<'_>> {
    map(
        tuple((
            tag("<!ATTLIST"),
            xml_ws,
            xml_name,
            many0(map(
                tuple((
                    xml_ws,
                    xml_name,
                    xml_ws,
                    xml_att_type,
                    xml_ws,
                    xml_default_decl,
                )),
                |(_, name, _, att_type, _, default_decl)| XmlAttDef {
                    name: name.0,
                    att_type,
                    default_decl,
                },
            )),
            opt(xml_ws),
            tag(">"),
        )),
        |(_, _, name, att_defs, _, _)| XmlAttListDecl {
            name: name.0,
            att_defs,
        },
    )(value)
}

/// Parse xml `Mixed` token.
pub fn xml_mixed(value: &str) -> IResult<&str, XmlMixed<'_>> {
    let (value, _) = satisfy(|c| c == '(')(value)?;

    let (value, _) = opt(xml_ws)(value)?;

    let (value, _) = tag("#PCDATA")(value)?;

    let (value, _) = opt(xml_ws)(value)?;

    let (value, list) = opt(tuple((opt(xml_ws), satisfy(|c| c == '|'), opt(xml_ws))))(value)?;

    let (value, types) = if list.is_some() {
        let (value, types) = separated_list0(
            tuple((opt(xml_ws), satisfy(|c| c == '|'), opt(xml_ws))),
            xml_decl_name,
        )(value)?;

        (value, Some(types))
    } else {
        (value, None)
    };

    let (value, _) = opt(xml_ws)(value)?;

    let value = if types.is_none() {
        let (value, _) = satisfy(|c| c == ')')(value)?;
        value
    } else {
        let (value, _) = tag(")*")(value)?;
        value
    };

    Ok((value, XmlMixed(types)))
}

/// Parse xml `DeclName` token.
pub fn xml_decl_name(value: &str) -> IResult<&str, XmlDeclName<'_>> {
    alt((
        map(xml_peref, |v| XmlDeclName::PEReference(v.0)),
        map(xml_name, |v| XmlDeclName::Name(v.0)),
    ))(value)
}

/// Parse xml `Repeat` token.
pub fn xml_repeat(value: &str) -> IResult<&str, XmlRepeat> {
    alt((
        map(tag("?"), |_| XmlRepeat::ZeroOrOne),
        map(tag("*"), |_| XmlRepeat::ZeroOrMany),
        map(tag("+"), |_| XmlRepeat::OneOrMany),
    ))(value)
}

/// Parse xml `cp` token.
pub fn xml_cp(value: &str) -> IResult<&str, XmlCP<'_>> {
    alt((
        map(xml_decl_name, |v| XmlCP::Name(v)),
        map(xml_children, |v| XmlCP::Children(v)),
    ))(value)
}

/// Parse xml `children` token.
pub fn xml_children(value: &str) -> IResult<&str, XmlChildren<'_>> {
    alt((
        map(tuple((xml_choice, opt(xml_repeat))), |(cps, repeat)| {
            XmlChildren::Choice { cps, repeat }
        }),
        map(tuple((xml_seq, opt(xml_repeat))), |(cps, repeat)| {
            XmlChildren::Seq { cps, repeat }
        }),
    ))(value)
}

/// Parse xml `children` token.
pub fn xml_choice(value: &str) -> IResult<&str, Vec<XmlCP<'_>>> {
    delimited(
        tuple((tag("("), opt(xml_ws))),
        separated_list1(tuple((opt(xml_ws), tag("|"), opt(xml_ws))), xml_cp),
        tuple((opt(xml_ws), tag(")"))),
    )(value)
}

/// Parse xml `children` token.
pub fn xml_seq(value: &str) -> IResult<&str, Vec<XmlCP<'_>>> {
    delimited(
        tuple((tag("("), opt(xml_ws))),
        separated_list0(tuple((opt(xml_ws), tag(","), opt(xml_ws))), xml_cp),
        tuple((opt(xml_ws), tag(")"))),
    )(value)
}

/// Parse xml `children` token.
pub fn xml_element_decl(value: &str) -> IResult<&str, XmlElementDecl<'_>> {
    map(
        tuple((
            tag("<!ELEMENT"),
            xml_ws,
            xml_decl_name,
            xml_ws,
            alt((
                map(tag("EMPTY"), |_| XmlContentSpec::Empty),
                map(tag("ANY"), |_| XmlContentSpec::Any),
                map(xml_peref, |v| XmlContentSpec::PEReference(v.0)),
                map(xml_mixed, |v| XmlContentSpec::Mixed(v)),
                map(xml_children, |v| XmlContentSpec::Children(v)),
            )),
            opt(xml_ws),
            tag(">"),
        )),
        |(_, _, name, _, content, _, _)| XmlElementDecl { name, content },
    )(value)
}

/// Parse xml `doctypedecl` token.
pub fn xml_doctype_decl(value: &str) -> IResult<&str, XmlDocTypeDecl<'_>> {
    map(
        tuple((
            tag("<!DOCTYPE"),
            xml_ws,
            xml_name,
            opt(tuple((xml_ws, xml_external_id))),
            opt(xml_ws),
            opt(delimited(
                tag("["),
                many0(alt((
                    map(xml_decl_sep, |v| match v {
                        XmlDeclSep::PEReference(v) => XmlMarkupDecl::PEReference(v),
                        XmlDeclSep::Space(v) => XmlMarkupDecl::Space(v),
                    }),
                    map(xml_element_decl, |v| XmlMarkupDecl::ElementDecl(v)),
                    map(xml_att_list_decl, |v| XmlMarkupDecl::AttListDecl(v)),
                    map(xml_entity_decl, |v| XmlMarkupDecl::EntityDecl(v)),
                    map(xml_notation_decl, |v| XmlMarkupDecl::Notation(v)),
                    map(xml_pi, |v| XmlMarkupDecl::PI(v)),
                    map(xml_comment, |v| XmlMarkupDecl::Comment(v)),
                ))),
                tag("]"),
            )),
            opt(xml_ws),
            tag(">"),
        )),
        |(_, _, name, external_id, _, int_subset, _, _)| XmlDocTypeDecl {
            name: name.0,
            external_id: external_id.map(|(_, id)| id),
            int_subset,
        },
    )(value)
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
    fn system_lit() {
        let (_, value) = xml_system_lit("'\u{2122}' ").unwrap();

        assert_eq!(value, XmlSystemLiteral("\u{2122}"));

        let (_, value) = xml_system_lit(r#"'"hello' "#).unwrap();

        assert_eq!(value, XmlSystemLiteral("\"hello"));

        let (_, value) = xml_system_lit(r#""" "#).unwrap();

        assert_eq!(value, XmlSystemLiteral(""));

        let (_, value) = xml_system_lit(r#""'hello" "#).unwrap();

        assert_eq!(value, XmlSystemLiteral("'hello"));
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
                XmlEntityValuePart::CharRef('™'),
                XmlEntityValuePart::Literal(" world "),
                XmlEntityValuePart::EntityRef("hello")
            ]
        );
    }

    #[test]
    fn attr_value() {
        let (_, value) = xml_att_value(r#"' hello &#x2122; world &hello;' "#).unwrap();

        assert_eq!(
            value,
            vec![
                XmlAttValuePart::Literal(" hello "),
                XmlAttValuePart::CharRef('™'),
                XmlAttValuePart::Literal(" world "),
                XmlAttValuePart::EntityRef("hello")
            ]
        );

        xml_att_value(r#"'%hello:a; hello &#x2122; world &hello;' "#)
            .expect_err("AttValue: unspport PEReference");
    }

    #[test]
    fn comment() {
        let (_, value) = xml_comment("<!-- hello - world& --> ").unwrap();

        assert_eq!(value, XmlComment(" hello - world& "));

        let (_, value) = xml_comment("<!-- declarations for <head> & <body> --> ").unwrap();

        assert_eq!(value, XmlComment(" declarations for <head> & <body> "));

        assert_eq!(
            xml_comment("<!-- B+, B, or B---> "),
            Err(nom::Err::Error(Error::new(
                "---> ",
                nom::error::ErrorKind::Tag
            )))
        );

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

        let (_, value) = xml_pi("<?xml-stylesheet   &#160; hello<>world   ?> ").unwrap();

        assert_eq!(
            value,
            XmlPI {
                target: "xml-stylesheet",
                data: Some("&#160; hello<>world   ")
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
        assert_eq!(
            xml_decl(r#"<?xml version="1.0"?> "#),
            Ok((
                " ",
                XmlDecl {
                    version: XmlVersion::Version1_0,
                    encoding: None,
                    standalone: None
                }
            ))
        );

        assert_eq!(
            xml_decl(r#"<?xml version="1.0" encoding="utf-8"?> "#),
            Ok((
                " ",
                XmlDecl {
                    version: XmlVersion::Version1_0,
                    encoding: Some("utf-8"),
                    standalone: None
                }
            ))
        );

        assert_eq!(
            xml_decl(r#"<?xml version="1.1"  standalone="yes"       ?> "#),
            Ok((
                " ",
                XmlDecl {
                    version: XmlVersion::Version1_1,
                    encoding: None,
                    standalone: Some(true)
                }
            ))
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
    fn public_id() {
        let (_, value) = xml_public_id("PUBLIC '' ").unwrap();

        assert_eq!(value, XmlPublicId(""));

        let (_, value) = xml_public_id(r#"PUBLIC "'hello" "#).unwrap();

        assert_eq!(value, XmlPublicId("'hello"));

        assert_eq!(
            xml_public_id("PUBLIC '\u{2122}' "),
            Err(nom::Err::Error(Error::new(
                "\u{2122}' ",
                nom::error::ErrorKind::Satisfy
            )))
        );
    }

    #[test]
    fn notation_decl() {
        assert_eq!(
            xml_notation_decl(r#"<!NOTATION PNG SYSTEM "image/png"> "#),
            Ok((
                " ",
                XmlNotationDecl {
                    name: "PNG",
                    id: XmlNotationId::System("image/png")
                }
            ))
        );

        assert_eq!(
            xml_notation_decl(
                r#"<!NOTATION PNG PUBLIC "-//Textuality//TEXT Standard open-hatch boilerplate//EN" 
                "http://www.textuality.com/boilerplate/OpenHatch.xml" > "#
            ),
            Ok((
                " ",
                XmlNotationDecl {
                    name: "PNG",
                    id: XmlNotationId::PublicSystem {
                        public_id: "-//Textuality//TEXT Standard open-hatch boilerplate//EN",
                        system_id: "http://www.textuality.com/boilerplate/OpenHatch.xml"
                    }
                }
            ))
        );
    }

    #[test]
    fn ndata_decl() {
        assert_eq!(
            xml_ndata_decl(r#" NDATA gif "#),
            Ok((" ", XmlNDataDecl("gif")))
        );
    }

    #[test]
    fn entity_decl() {
        assert_eq!(
            xml_entity_decl(
                r#"<!ENTITY picture SYSTEM "http://www.somesite.com/somePic.gif" NDATA GIF> "#
            ),
            Ok((
                " ",
                XmlEntityDecl::GEDecl {
                    name: "picture",
                    def: XmlEntityDef::External {
                        id: XmlExternalId::System("http://www.somesite.com/somePic.gif"),
                        ndata_decl: Some("GIF")
                    }
                }
            ))
        );

        assert_eq!(
            xml_entity_decl(
                r#"<!ENTITY Pub-Status "This is a pre-release of the specification."> "#
            ),
            Ok((
                " ",
                XmlEntityDecl::GEDecl {
                    name: "Pub-Status",
                    def: XmlEntityDef::Value(vec![XmlEntityValuePart::Literal(
                        "This is a pre-release of the specification."
                    )])
                }
            ))
        );
    }

    #[test]
    fn enum_type() {
        assert_eq!(
            xml_enum_type(r#"NOTATION (js|cs|perl) "#),
            Ok((" ", XmlEnumType::Notation(vec!["js", "cs", "perl"])))
        );

        assert_eq!(
            xml_enum_type(r#"(apple | pear | bannan) "#),
            Ok((" ", XmlEnumType::NmToken(vec!["apple", "pear", "bannan"])))
        );
    }

    #[test]
    fn tokenized_type() {
        assert_eq!(
            xml_tokenized_type(r#"ID "#),
            Ok((" ", XmlTokenizedType::Id))
        );

        assert_eq!(
            xml_tokenized_type(r#"IDREF "#),
            Ok((" ", XmlTokenizedType::IdRef))
        );

        assert_eq!(
            xml_tokenized_type(r#"IDREFS "#),
            Ok((" ", XmlTokenizedType::IdRefs))
        );

        assert_eq!(
            xml_tokenized_type(r#"ENTITY "#),
            Ok((" ", XmlTokenizedType::Entity))
        );

        assert_eq!(
            xml_tokenized_type(r#"ENTITIES "#),
            Ok((" ", XmlTokenizedType::Entities))
        );

        assert_eq!(
            xml_tokenized_type(r#"NMTOKEN "#),
            Ok((" ", XmlTokenizedType::NmToken))
        );

        assert_eq!(
            xml_tokenized_type(r#"NMTOKENS "#),
            Ok((" ", XmlTokenizedType::NmTokens))
        );
    }

    #[test]
    fn att_type() {
        assert_eq!(
            xml_att_type(r#"NMTOKENS "#),
            Ok((" ", XmlAttType::Tokenized(XmlTokenizedType::NmTokens)))
        );

        assert_eq!(xml_att_type(r#"CDATA "#), Ok((" ", XmlAttType::String)));

        assert_eq!(
            xml_att_type(r#"NOTATION (js|cs|perl) "#),
            Ok((" ", XmlAttType::Notation(vec!["js", "cs", "perl"])))
        );

        assert_eq!(
            xml_att_type(r#"(apple | pear | bannan) "#),
            Ok((" ", XmlAttType::NmToken(vec!["apple", "pear", "bannan"])))
        );
    }

    #[test]
    fn default_decl() {
        assert_eq!(
            xml_default_decl(r#"#REQUIRED "#),
            Ok((" ", XmlDefaultDecl::Required))
        );

        assert_eq!(
            xml_default_decl(r#"#IMPLIED "#),
            Ok((" ", XmlDefaultDecl::Implied))
        );

        assert_eq!(
            xml_default_decl(r#"#FIXED   ' hello &#x2122; world &hello;' "#),
            Ok((
                " ",
                XmlDefaultDecl::Fixed(vec![
                    XmlAttValuePart::Literal(" hello "),
                    XmlAttValuePart::CharRef('™'),
                    XmlAttValuePart::Literal(" world "),
                    XmlAttValuePart::EntityRef("hello")
                ])
            ))
        );
    }

    #[test]
    fn att_list_decl() {
        assert_eq!(
            xml_att_list_decl(r#"<!ATTLIST test myAttr CDATA #IMPLIED> "#),
            Ok((
                " ",
                XmlAttListDecl {
                    name: "test",
                    att_defs: vec![XmlAttDef {
                        name: "myAttr",
                        att_type: XmlAttType::String,
                        default_decl: XmlDefaultDecl::Implied
                    }]
                }
            ))
        );

        assert_eq!(
            xml_att_list_decl(r#"<!ATTLIST test fruit (apple | pear | bannan) #REQUIRED> "#),
            Ok((
                " ",
                XmlAttListDecl {
                    name: "test",
                    att_defs: vec![XmlAttDef {
                        name: "fruit",
                        att_type: XmlAttType::NmToken(vec!["apple", "pear", "bannan"]),
                        default_decl: XmlDefaultDecl::Required
                    }]
                }
            ))
        );

        assert_eq!(
            xml_att_list_decl(
                r#"<!ATTLIST test fruit (apple | pear | bannan) #REQUIRED     
                myAttr CDATA #IMPLIED > "#
            ),
            Ok((
                " ",
                XmlAttListDecl {
                    name: "test",
                    att_defs: vec![
                        XmlAttDef {
                            name: "fruit",
                            att_type: XmlAttType::NmToken(vec!["apple", "pear", "bannan"]),
                            default_decl: XmlDefaultDecl::Required
                        },
                        XmlAttDef {
                            name: "myAttr",
                            att_type: XmlAttType::String,
                            default_decl: XmlDefaultDecl::Implied
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn mixed() {
        let (_, mixed) = xml_mixed(r#"(#PCDATA) "#).unwrap();

        assert_eq!(mixed, XmlMixed(None));

        assert_eq!(xml_mixed(r#"(#PCDATA)* "#), Ok(("* ", XmlMixed(None))));

        let (_, mixed) = xml_mixed(r#"(#PCDATA|a|ul|b|i|em)* "#).unwrap();

        assert_eq!(
            mixed,
            XmlMixed(Some(vec![
                XmlDeclName::Name("a"),
                XmlDeclName::Name("ul"),
                XmlDeclName::Name("b"),
                XmlDeclName::Name("i"),
                XmlDeclName::Name("em")
            ]))
        );

        let (_, mixed) =
            xml_mixed(r#"(#PCDATA | %font; | %phrase; | %special; | %form;)* "#).unwrap();

        assert_eq!(
            mixed,
            XmlMixed(Some(vec![
                XmlDeclName::PEReference("font"),
                XmlDeclName::PEReference("phrase"),
                XmlDeclName::PEReference("special"),
                XmlDeclName::PEReference("form"),
            ]))
        );
    }

    #[test]
    fn element_decl() {
        assert_eq!(
            xml_element_decl(r#"<!ELEMENT br EMPTY> "#),
            Ok((
                " ",
                XmlElementDecl {
                    name: XmlDeclName::Name("br"),
                    content: XmlContentSpec::Empty
                }
            ))
        );

        assert_eq!(
            xml_element_decl(r#"<!ELEMENT p (#PCDATA|emph)* > "#),
            Ok((
                " ",
                XmlElementDecl {
                    name: XmlDeclName::Name("p"),
                    content: XmlContentSpec::Mixed(XmlMixed(Some(vec![XmlDeclName::Name("emph")])))
                }
            ))
        );

        assert_eq!(
            xml_element_decl(r#"<!ELEMENT %name.para; %content.para; > "#),
            Ok((
                " ",
                XmlElementDecl {
                    name: XmlDeclName::PEReference("name.para"),
                    content: XmlContentSpec::PEReference("content.para")
                }
            ))
        );

        assert_eq!(
            xml_element_decl(r#"<!ELEMENT container ANY> "#),
            Ok((
                " ",
                XmlElementDecl {
                    name: XmlDeclName::Name("container"),
                    content: XmlContentSpec::Any
                }
            ))
        );
    }

    #[test]
    fn doctype_decl() {
        assert_eq!(
            xml_doctype_decl(
                r#"<!DOCTYPE note [
                                    <!ELEMENT note (to,from,heading,body)>
                                    <!ELEMENT to (#PCDATA)>
                                    <!ELEMENT from (#PCDATA)>
                                    <!ELEMENT heading (#PCDATA)>
                                    <!ELEMENT body (#PCDATA)>
                ]>
                "#
            ),
            Ok((
                "\n                ",
                XmlDocTypeDecl {
                    name: "note",
                    external_id: None,
                    int_subset: Some(vec![
                        XmlMarkupDecl::Space("\n                                    "),
                        XmlMarkupDecl::ElementDecl(XmlElementDecl {
                            name: XmlDeclName::Name("note"),
                            content: XmlContentSpec::Children(XmlChildren::Seq {
                                cps: vec![
                                    XmlCP::Name(XmlDeclName::Name("to")),
                                    XmlCP::Name(XmlDeclName::Name("from")),
                                    XmlCP::Name(XmlDeclName::Name("heading")),
                                    XmlCP::Name(XmlDeclName::Name("body"))
                                ],
                                repeat: None
                            })
                        }),
                        XmlMarkupDecl::Space("\n                                    "),
                        XmlMarkupDecl::ElementDecl(XmlElementDecl {
                            name: XmlDeclName::Name("to"),
                            content: XmlContentSpec::Mixed(XmlMixed(None))
                        }),
                        XmlMarkupDecl::Space("\n                                    "),
                        XmlMarkupDecl::ElementDecl(XmlElementDecl {
                            name: XmlDeclName::Name("from"),
                            content: XmlContentSpec::Mixed(XmlMixed(None))
                        }),
                        XmlMarkupDecl::Space("\n                                    "),
                        XmlMarkupDecl::ElementDecl(XmlElementDecl {
                            name: XmlDeclName::Name("heading"),
                            content: XmlContentSpec::Mixed(XmlMixed(None))
                        }),
                        XmlMarkupDecl::Space("\n                                    "),
                        XmlMarkupDecl::ElementDecl(XmlElementDecl {
                            name: XmlDeclName::Name("body"),
                            content: XmlContentSpec::Mixed(XmlMixed(None))
                        }),
                        XmlMarkupDecl::Space("\n                "),
                    ])
                }
            ))
        );
    }
}
