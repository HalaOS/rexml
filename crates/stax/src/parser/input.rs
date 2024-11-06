use core::{future::Future, str};

use std::future::Ready;

/// Result of [`lookahead`] function.
pub enum Lookahead {
    /// Cached new data with length.
    Buffered(usize),
    /// Input stream's cache buff overflowed.
    Overflow(usize),
    /// The input stream is closed by upstream.
    BrokenPipe,
}

/// A parser input stream must implement this trait.
pub trait InputStream {
    /// Opaque to check stream position.
    type Cursor: PartialEq;

    /// A future created by [`lookahead`](InputStream::lookahead) function.
    type Lookahead<'a>: Future<Output = Lookahead>
    where
        Self: 'a;

    /// Returns the lookahead buf length.
    fn len(&self) -> usize;

    /// Returns the slice of lookahead buf.
    fn slice(&self) -> &[u8];

    /// Returns lookahead buf as &str.
    fn as_str(&self) -> &str;

    /// Load new data up to `len` from the upstream.
    fn lookahead(&mut self, len: usize) -> Self::Lookahead<'_>;

    /// Move lookahead buffer cursor with `steps` bytes.
    ///
    /// The argument, mid, should be a byte offset from the start of the string.
    /// it must also be on the boundary of a UTF-8 code point for some impls.
    fn advance(&mut self, steps: usize);

    /// Returns the current position of this input stream.
    fn position(&self) -> Self::Cursor;
}

/// An extension trait that convert self into [`InputStream`]
pub trait IntoInputStream {
    /// Target `InputStream`.
    type InputStream: InputStream;

    /// Convert self into [`InputStream`](IntoInputStream::InputStream)
    fn into_input_stream(self) -> Self::InputStream;
}

impl<T> IntoInputStream for T
where
    T: InputStream,
{
    type InputStream = T;

    fn into_input_stream(self) -> Self::InputStream {
        self
    }
}

impl InputStream for &str {
    type Cursor = usize;

    type Lookahead<'a>
        = Ready<Lookahead>
    where
        Self: 'a;

    fn len(&self) -> usize {
        str::len(&self)
    }

    fn slice(&self) -> &[u8] {
        str::as_bytes(&self)
    }

    fn as_str(&self) -> &str {
        self
    }

    fn lookahead(&mut self, _len: usize) -> Self::Lookahead<'_> {
        // always return BrokenPipe.
        std::future::ready(Lookahead::BrokenPipe)
    }

    fn advance(&mut self, steps: usize) {
        let (_, last) = self.split_at(steps);

        *self = last;
    }

    fn position(&self) -> Self::Cursor {
        str::len(&self)
    }
}

impl IntoInputStream for String {
    type InputStream = (String, usize);

    fn into_input_stream(self) -> Self::InputStream {
        (self, 0)
    }
}

impl InputStream for (String, usize) {
    type Cursor = usize;

    type Lookahead<'a>
        = Ready<Lookahead>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.0.len() - self.1
    }

    fn slice(&self) -> &[u8] {
        &self.0.as_bytes()[self.1..]
    }

    fn as_str(&self) -> &str {
        // Safety: change the offset value by advance function.
        unsafe { str::from_utf8_unchecked(self.slice()) }
    }

    fn lookahead(&mut self, _len: usize) -> Self::Lookahead<'_> {
        // always return BrokenPipe.
        std::future::ready(Lookahead::BrokenPipe)
    }

    fn advance(&mut self, steps: usize) {
        assert!(self.len() >= steps);

        self.1 += steps;
    }

    fn position(&self) -> Self::Cursor {
        self.1
    }
}
