//! This mod defines input traits for parsers.

/// A parser input stream must implement this trait.
pub trait InputStream {
    type Item;
    type Slice<'a>
    where
        Self: 'a;

    fn slice(&self) -> Self::Slice<'_>;
}

impl InputStream for &str {
    type Item = char;
    type Slice<'a> = &'a str where Self: 'a;

    fn slice(&self) -> Self::Slice<'_> {
        self
    }
}
