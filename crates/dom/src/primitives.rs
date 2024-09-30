use std::{borrow::Cow, fmt::Display};

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
    /// This is an extension type and is not part of the DOM standard.
    Namespace = 13,
}

/// This corresponds to the DOM Node interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeRef {
    /// The reference id of the memory manager of the document to which this node belongs.
    id: usize,
    /// The [`node_type`](NodeType) of this node.
    pub node_type: NodeType,
}

impl NodeRef {
    #[allow(unused)]
    pub(crate) fn new(id: usize, node_type: NodeType) -> Self {
        Self { id, node_type }
    }
}

impl Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.node_type, self.id,)
    }
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
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let (input, qname) = parse_qname(value).map_err(|_| crate::Error::QName(&value))?;

        if !input.is_empty() {
            return Err(crate::Error::QName(value));
        }

        Ok(qname)
    }
}

impl<'a> Display for QName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, "{}:{}", prefix, self.local_part)
        } else {
            write!(f, "{}", self.local_part)
        }
    }
}

/// This corresponds to the namespace extension.
#[derive(Debug)]
pub struct Namespace<'a> {
    /// The node to which this namespace belongs.
    node: NodeRef,
    /// The namespace prefix
    prefix: Cow<'a, str>,
    /// The namespace href
    href: Cow<'a, str>,
}

impl<'a> Namespace<'a> {
    #[allow(unused)]
    fn new(node: NodeRef, prefix: Cow<'a, str>, href: Cow<'a, str>) -> Self {
        Self { node, prefix, href }
    }

    /// Return namespace's parent node.
    pub fn parent(&self) -> &NodeRef {
        &self.node
    }

    /// Returns the prefix part of namespace
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns the href part of namespace
    pub fn href(&self) -> &str {
        &self.href
    }
}

impl<'a> Display for Namespace<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Namespace({},{},{})", self.node, self.prefix, self.href,)
    }
}

#[cfg(test)]
mod tests {
    use crate::Namespace;

    use super::{NodeRef, NodeType, QName};

    #[test]
    fn test_qname() {
        let qname: QName = "h:hello".try_into().unwrap();

        assert_eq!(qname.to_string(), "h:hello");

        QName::try_from("-hello").expect_err("NameStartChar");

        let _: QName = "_h:hello".try_into().unwrap();

        let qname: QName = "hello".try_into().unwrap();

        assert_eq!(qname.to_string(), "hello");
    }

    #[test]
    fn test_node() {
        println!("{}", NodeRef::new(1, NodeType::Attribute));
    }

    #[test]
    fn test_namespace() {
        println!(
            "{}",
            Namespace::new(
                NodeRef::new(1, NodeType::Element),
                "xsl".into(),
                "http://www.w3.org/1999/XSL/Transform".into()
            )
        );
    }
}
