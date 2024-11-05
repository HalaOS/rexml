#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("parse failed, {0}")]
    ParseFailed(ParserKind),

    #[error("input stream broken, {0}")]
    BrokenPipe(ParserKind),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParserKind {
    #[error("safisfy(chars)")]
    Safisfy,

    #[error("none_of(chars)")]
    NoneOf,

    #[error("one_of(chars)")]
    OneOf,

    #[error("Tag(bytes)")]
    Tag,

    #[error("Search(bytes)")]
    Search,
}
