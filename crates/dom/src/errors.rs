use crate::NodeRef;

/// Error type returns by this mod.
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("Invalid NCName: {0}")]
    QName(&'a str),

    #[error("Not found node: {0}")]
    NodeNotFound(NodeRef),
}

/// Result type returns by this mod.
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
