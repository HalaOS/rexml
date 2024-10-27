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
pub enum XmlAttValuePart<'a> {
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

/// A token represents xml/1.1 `NDataDecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlNDataDecl<'a>(pub &'a str);

/// A token represents xml/1.1 `DeclSep`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlDeclSep<'a> {
    PEReference(&'a str),
    Space(&'a str),
}

/// A token represents xml/1.1 `Pubid`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlPublicId<'a>(pub &'a str);

/// A token represents xml/1.1 `ElementDecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlNotationDecl<'a> {
    pub name: &'a str,
    pub id: XmlNotationId<'a>,
}

/// A token represents xml/1.1 `NotationId`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlNotationId<'a> {
    System(&'a str),
    PublicSystem {
        public_id: &'a str,
        system_id: &'a str,
    },
    Public(&'a str),
}

/// A token represents xml/1.1 `EntityDecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlEntityDecl<'a> {
    GEDecl {
        name: &'a str,
        def: XmlEntityDef<'a>,
    },

    PEDecl {
        name: &'a str,
        def: XmlPEDef<'a>,
    },
}

/// A token represents xml/1.1 `EntityDef`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlEntityDef<'a> {
    Value(Vec<XmlEntityValuePart<'a>>),
    External {
        id: XmlExternalId<'a>,
        ndata_decl: Option<&'a str>,
    },
}

/// A token represents xml/1.1 `PEDef`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlPEDef<'a> {
    Value(Vec<XmlEntityValuePart<'a>>),
    External(XmlExternalId<'a>),
}

/// A token represents xml/1.1 `EnumeratedType`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlEnumType<'a> {
    Notation(Vec<&'a str>),
    NmToken(Vec<&'a str>),
}

/// A token represents xml/1.1 `TokenizedType`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlTokenizedType {
    Id,
    IdRef,
    IdRefs,
    Entity,
    Entities,
    NmToken,
    NmTokens,
}

/// A token represents xml/1.1 `AttType`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlAttType<'a> {
    String,
    Tokenized(XmlTokenizedType),
    Notation(Vec<&'a str>),
    NmToken(Vec<&'a str>),
}

/// A token represents xml/1.1 `DefaultDecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlDefaultDecl<'a> {
    Required,
    Implied,
    Fixed(Vec<XmlAttValuePart<'a>>),
}

/// A token represents xml/1.1 `AttlistDecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlAttListDecl<'a> {
    pub name: &'a str,
    pub att_defs: Vec<XmlAttDef<'a>>,
}

/// A token represents xml/1.1 `AttDef`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlAttDef<'a> {
    pub name: &'a str,
    pub att_type: XmlAttType<'a>,
    pub default_decl: XmlDefaultDecl<'a>,
}

/// A token represents xml/1.1 `NotationId`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlMixed<'a>(pub Option<Vec<XmlDeclName<'a>>>);

/// A token represents xml/1.1 `DeclName`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlDeclName<'a> {
    Name(&'a str),
    PEReference(&'a str),
}

/// A token represents xml/1.1 `children`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlChildren<'a> {
    Choice {
        cps: Vec<XmlCP<'a>>,
        repeat: Option<XmlRepeat>,
    },
    Seq {
        cps: Vec<XmlCP<'a>>,
        repeat: Option<XmlRepeat>,
    },
}

/// A token represents xml/1.1 `cp`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlCP<'a> {
    Name(XmlDeclName<'a>),
    Children(XmlChildren<'a>),
}

/// A token represents xml/1.1 `Repeat`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlRepeat {
    ZeroOrOne,
    ZeroOrMany,
    OneOrMany,
}

/// A token represents xml/1.1 `contentspec`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlContentSpec<'a> {
    Empty,
    Any,
    Mixed(XmlMixed<'a>),
    Children(XmlChildren<'a>),
    PEReference(&'a str),
}

/// A token represents xml/1.1 `elementdecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlElementDecl<'a> {
    pub name: XmlDeclName<'a>,
    pub content: XmlContentSpec<'a>,
}

/// A token represents xml/1.1 `doctypedecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlDocTypeDecl<'a> {
    pub name: &'a str,
    pub external_id: Option<XmlExternalId<'a>>,
    pub int_subset: Option<Vec<XmlMarkupDecl<'a>>>,
}

/// A token represents xml/1.1 `markupdecl`.
///
/// See [`XML_EBNF1.1`] for more information.
///
/// [`XML_EBNF1.1`]: https://www.liquid-technologies.com/Reference/Glossary/XML_EBNF1.1.html
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum XmlMarkupDecl<'a> {
    ElementDecl(XmlElementDecl<'a>),
    AttListDecl(XmlAttListDecl<'a>),
    EntityDecl(XmlEntityDecl<'a>),
    Notation(XmlNotationDecl<'a>),
    PI(XmlPI<'a>),
    Comment(XmlComment<'a>),
    PEReference(&'a str),
    Space(&'a str),
}
