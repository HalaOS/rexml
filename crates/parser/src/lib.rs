//! A XML document parser backed with nom.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;
#[cfg(not(feature = "std"))]
use std::prelude::v1::*;

/// Error returns by the parser.
#[derive(Debug, thiserror::Error)]
pub enum SaxError {}

/// A parser backend handler to generate parsed result.
pub trait SaxHandler<'source> {
    type Error: std::error::Error + From<SaxError>;

    /// Receive notification of the beginning of the document.
    fn start_document(&mut self) -> Result<(), Self::Error>;

    /// Receive notification of the end of the document.
    fn end_document(&mut self) -> Result<(), Self::Error>;

    /// Receive notification of the start of an element.
    fn start_element(&mut self, qname: &'source str) -> Result<(), Self::Error>;

    ///  Receive notification of a `attribute` declaration.
    fn attr(&mut self, qname: &'source str, value: &'source str) -> Result<(), Self::Error>;

    /// Receive notification of a `text` node declaration.
    fn text(&mut self, content: &'source str) -> Result<(), Self::Error>;

    /// Receive notification of a `processing instruction` declaration.
    fn processing_instruction(
        &mut self,
        target: &'source str,
        data: &'source str,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `notation` declaration.
    fn notation_decl(
        &mut self,
        name: &'source str,
        public_id: &'source str,
        system_id: &'source str,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `unparsed entity`.
    fn unparsed_entity_decl(
        &mut self,
        name: &'source str,
        public_id: &'source str,
        system_id: &'source str,
        notation_name: &'source str,
    ) -> Result<(), Self::Error>;

    /// Receive notification of a `skipped entity`.
    fn skipped_entity(&mut self, name: &'source str) -> Result<(), Self::Error>;

    /// Resolve an external entity.
    fn resolve_entity(
        &mut self,
        public_id: &'source str,
        system_id: &'source str,
    ) -> Result<(), Self::Error>;

    /// Receive notification of the end of an element.
    fn end_element(&mut self, qname: &'source str) -> Result<(), Self::Error>;

    /// Receive notification of a parser warning.
    fn warning(&mut self, err: SaxError) -> Result<(), Self::Error>;

    /// Receive notification of a recoverable parser error.
    fn error(&mut self, err: SaxError) -> Result<(), Self::Error>;

    /// Report a fatal XML parsing error.
    fn fatal_error(&mut self, err: SaxError) -> Result<(), Self::Error>;
}
