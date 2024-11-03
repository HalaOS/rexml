//! This mod defines input traits for parsers.

/// A parser input stream must implement this trait.
pub trait InputStream {}

impl InputStream for &str {}
