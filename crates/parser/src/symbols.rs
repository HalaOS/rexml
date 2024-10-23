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
    Reference(XmlReference<'a>),
    PEReference(&'a str),
    Literal(&'a str),
}
