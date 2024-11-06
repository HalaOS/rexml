/// Error type raised by this crate.
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

    #[error("tag(bytes)")]
    Tag,

    #[error("search(bytes)")]
    Search,

    #[error("iter(combinator)")]
    Iter,
}

/// Result type used by this crate.
pub type Result<I, O, E> = std::result::Result<(I, O), (I, E)>;
