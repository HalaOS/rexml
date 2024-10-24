/// A token represents xml/1.1 `White Space`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlWhiteSpace<'a>(pub &'a str);

/// A token represents xml/1.1 `Name`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlName<'a>(pub &'a str);

/// A token represents xml/1.1 `XmlNmToken`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlNmToken<'a>(pub &'a str);

/// A token represents xml/1.1 `PubidLiteral`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlPubidLiteral<'a>(pub &'a str);

/// A token represents xml/1.1 `SystemLiteral`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlSystemLiteral<'a>(pub &'a str);

/// A token represents xml/1.1 `Reference`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlReference<'a> {
    Char(char),
    Entity(&'a str),
}

/// A token represents xml/1.1 `PEReference`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlPEReference<'a>(pub &'a str);

/// A token represents xml/1.1 `EntityValue`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlEntityValuePart<'a> {
    CharRef(char),
    EntityRef(&'a str),
    PEReference(&'a str),
    Literal(&'a str),
}

/// A token represents xml/1.1 `AttrValue`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlAttrValuePart<'a> {
    CharRef(char),

    EntityRef(&'a str),

    Literal(&'a str),
}

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

/// A token represents xml/1.1 `CData`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlCData<'a>(pub &'a str);

/// A token represents xml/1.1 `Encoding`.
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

/// A token represents xml/1.1 `Version`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlVersion {
    Version1_0,
    Version1_1,
}

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

/// A token represents xml/1.1 `Misc`.
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
    Space(&'a str),
}
