use crate::DOMObject;

/// Error code corresponds to DOM `ExceptionCode`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DOMExceptionCode {
    INDEX_SIZE_ERR = 1,
    DOMSTRING_SIZE_ERR = 2,
    HIERARCHY_REQUEST_ERR = 3,
    WRONG_DOCUMENT_ERR = 4,
    INVALID_CHARACTER_ERR = 5,
    NO_DATA_ALLOWED_ERR = 6,
    NO_MODIFICATION_ALLOWED_ERR = 7,
    NOT_FOUND_ERR = 8,
    NOT_SUPPORTED_ERR = 9,
    INUSE_ATTRIBUTE_ERR = 10,
    INVALID_STATE_ERR = 11,
    SYNTAX_ERR = 12,
    INVALID_MODIFICATION_ERR = 13,
    NAMESPACE_ERR = 14,
    INVALID_ACCESS_ERR = 15,
}

/// Error type returns by this mod.
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("Invalid NCName: {0}")]
    QName(&'a str),

    #[error("Not found node: {0}")]
    NodeNotFound(DOMObject),

    #[error("DOMException: {0:?}")]
    DOMException(DOMExceptionCode),
}

/// Result type returns by this mod.
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
