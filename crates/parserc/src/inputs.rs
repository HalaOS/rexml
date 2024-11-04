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

/// A extension trait that convert `self` into [`InputStream`].
pub trait IntoInputStream {
    type Stream: InputStream;

    fn into_input_stream(self) -> Self::Stream;
}

/// Implement [`IntoInputStream`] for all [`InputStream`] types.
impl<T> IntoInputStream for T
where
    T: InputStream,
{
    type Stream = T;
    fn into_input_stream(self) -> Self::Stream {
        self
    }
}

impl<C, I> InputStream for (C, I)
where
    I: InputStream,
{
    type Item = I::Item;

    type Slice<'a> = I::Slice<'a> where Self: 'a;

    fn slice(&self) -> Self::Slice<'_> {
        self.1.slice()
    }
}
