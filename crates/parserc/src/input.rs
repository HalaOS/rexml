use std::future::Future;

/// Result of [`lookahead`](InputStream::lookahead) function.
pub enum Lookahead {
    /// Cached new data with length.
    Buffered(usize),
    /// Input stream's cache buff overflowed.
    Overflow(usize),
    /// The input stream is closed by upstream.
    BrokenPipe,
}

/// Input stream for parsers.
pub trait InputStream: Send {
    /// The Position type returns by [`position`](InputStream::position) function.
    type Cursor: PartialEq + Send;

    /// Returns the lookahead buf length.
    fn len(&self) -> usize;

    /// Returns the position cursor of this stream.
    fn position(&self) -> Self::Cursor;

    /// Returns the slice of lookahead buf.
    fn slice(&self) -> &[u8];

    /// Load more data from the upstream into the buffer
    fn lookahead(&mut self, delta: usize) -> impl Future<Output = Lookahead> + Send;

    /// The argument, mid, should be a byte offset from the start of the string.
    /// it must also be on the boundary of a UTF-8 code point for some impls.
    fn split_at(self, mid: usize) -> Self;
}

/// A input stream that data is encoded as utf-8 string.
pub trait AsStr {
    /// Returns lookahead buf as &str.
    fn as_str(&self) -> &str;
}

impl InputStream for &str {
    type Cursor = usize;
    fn len(&self) -> usize {
        str::len(&self)
    }

    fn slice(&self) -> &[u8] {
        str::as_bytes(&self)
    }

    fn lookahead(&mut self, _len: usize) -> impl Future<Output = Lookahead> {
        // always return BrokenPipe.
        async { Lookahead::BrokenPipe }
    }

    fn split_at(self, mid: usize) -> Self {
        let (_, last) = self.split_at(mid);

        last
    }

    fn position(&self) -> Self::Cursor {
        str::len(&self)
    }
}

impl AsStr for &str {
    fn as_str(&self) -> &str {
        self
    }
}
