use std::borrow::Cow;

use nom::Err;

use crate::Error;

/// A parser backend handler to generate parsed result.
pub trait SaxHandler<'source> {
    type Error: std::error::Error + Into<Err<Error<'source>>>;

    /// Receive notification of the beginning of the document.
    fn start_document(&mut self) -> Result<(), Self::Error>;

    /// Receive notification of the end of the document.
    fn end_document(&mut self) -> Result<(), Self::Error>;

    /// Receive notification of the start of an element.
    fn start_element(&mut self, qname: Cow<'source, str>) -> Result<(), Self::Error>;

    ///  Receive notification of a `attribute` declaration.
    fn attr(
        &mut self,
        qname: Cow<'source, str>,
        value: Cow<'source, str>,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `text` node declaration.
    fn text(&mut self, content: Cow<'source, str>) -> Result<(), Self::Error>;

    /// Receive notification of a `processing instruction` declaration.
    fn processing_instruction(
        &mut self,
        target: Cow<'source, str>,
        data: Option<Cow<'source, str>>,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `notation` declaration.
    fn notation_decl(
        &mut self,
        name: Cow<'source, str>,
        public_id: Cow<'source, str>,
        system_id: Cow<'source, str>,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `unparsed entity`.
    fn unparsed_entity_decl(
        &mut self,
        name: Cow<'source, str>,
        public_id: Cow<'source, str>,
        system_id: Cow<'source, str>,
        notation_name: Cow<'source, str>,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `skipped entity`.
    fn skipped_entity(&mut self, name: Cow<'source, str>) -> Result<(), Self::Error>;

    /// Resolve an external entity.
    fn resolve_entity(
        &mut self,
        public_id: Cow<'source, str>,
        system_id: Cow<'source, str>,
    ) -> Result<(), Self::Error>;

    /// Receive notification of the end of an element.
    fn end_element(&mut self, qname: Cow<'source, str>) -> Result<(), Self::Error>;

    /// Receive notification of a parser warning.
    fn warning(&mut self, err: Error) -> Result<(), Self::Error>;

    /// Receive notification of a recoverable parser error.
    fn error(&mut self, err: Error) -> Result<(), Self::Error>;

    /// Report a fatal XML parsing error.
    fn fatal_error(&mut self, err: Error) -> Result<(), Self::Error>;
}
