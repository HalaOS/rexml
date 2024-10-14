/// Error returns by the parser.
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("{0}")]
    NomError(nom::error::Error<&'a str>),
}

impl<'a> nom::error::ParseError<&'a str> for Error<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Error::NomError(nom::error::Error::new(input, kind))
    }

    fn append(input: &'a str, kind: nom::error::ErrorKind, _other: Self) -> Self {
        Error::NomError(nom::error::Error::new(input, kind))
    }
}

impl<'a> From<Error<'a>> for nom::Err<Error<'a>> {
    fn from(value: Error<'a>) -> Self {
        Self::Failure(value)
    }
}
