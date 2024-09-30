use std::borrow::Cow;

/// This corresponds to the DOM NodeType set of constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CData = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
}

/// A [`QName`], or qualified name, is the fully qualified name of an element, attribute, or identifier in an XML document.
///
/// [`QName`]: https://www.wikiwand.com/en/articles/QName
#[derive(Debug)]
pub struct QName<'a> {
    /// The prefix name of qualified name.
    pub prefix: Option<Cow<'a, str>>,
    /// The local part of qualified name.
    pub local_part: Cow<'a, str>,
}

impl<'a> TryFrom<&'a str> for QName<'a> {
    type Error = crate::Error<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use bytes::complete::*;
        use character::complete::*;
        use combinator::opt;
        use nom::*;
        fn nc_name(value: &str) -> IResult<&str, &str> {
            let (input, _) = satisfy(is_name_start_char)(value)?;

            let (input, body) = take_while(is_name_char)(input)?;

            let (name, _) = value.split_at(1 + body.len());

            Ok((input, name))
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
            c == '_'
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

        fn parse_qname(input: &str) -> IResult<&str, QName<'_>> {
            let (input, prefix_or_local_part) = nc_name(input)?;

            let (input, split) = opt(satisfy(|c| c == ':'))(input)?;

            if split.is_none() {
                return Ok((
                    input,
                    QName {
                        prefix: None,
                        local_part: prefix_or_local_part.into(),
                    },
                ));
            }

            let (input, local_part) = nc_name(input)?;

            return Ok((
                input,
                QName {
                    prefix: Some(prefix_or_local_part.into()),
                    local_part: local_part.into(),
                },
            ));
        }

        let (input, qname) = parse_qname(value).map_err(|_| crate::Error::QName(&value))?;

        if !input.is_empty() {
            return Err(crate::Error::QName(value));
        }

        Ok(qname)
    }
}

#[cfg(test)]
mod tests {
    use super::QName;

    #[test]
    fn test_qname() {
        let _: QName = "h:hello".try_into().unwrap();
        QName::try_from("-hello").expect_err("NameStartChar");

        let _: QName = "_h:hello".try_into().unwrap();
    }
}
