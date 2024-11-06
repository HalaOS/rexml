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
pub trait InputStream {
    /// The Position type returns by [`position`](InputStream::position) function.
    type Cursor: PartialEq;

    /// Returns the lookahead buf length.
    fn len(&self) -> usize;

    /// Returns the position cursor of this stream.
    fn position(&self) -> Self::Cursor;

    /// Returns the slice of lookahead buf.
    fn slice(&self) -> &[u8];

    /// Load more data from the upstream into the buffer
    fn lookahead(&mut self, delta: usize) -> impl Future<Output = Lookahead>;

    /// The argument, mid, should be a byte offset from the start of the string.
    /// it must also be on the boundary of a UTF-8 code point for some impls.
    fn split_at(self, mid: usize) -> Self;
}

/// A input stream that data is encoded as utf-8 string.
pub trait AsStr {
    /// Returns lookahead buf as &str.
    fn as_str(&self) -> &str;
}

impl<'a> InputStream for &'a str {
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

impl<'a> AsStr for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

impl<C, I> InputStream for (C, I)
where
    I: InputStream,
    C: Send,
{
    type Cursor = I::Cursor;

    fn len(&self) -> usize {
        self.1.len()
    }

    fn slice(&self) -> &[u8] {
        self.1.slice()
    }

    fn lookahead(&mut self, len: usize) -> impl Future<Output = Lookahead> {
        self.1.lookahead(len)
    }

    fn split_at(self, mid: usize) -> Self {
        let (c, i) = self;
        (c, i.split_at(mid))
    }

    fn position(&self) -> Self::Cursor {
        self.1.position()
    }
}

impl<C, I> AsStr for (C, I)
where
    I: AsStr,
    C: Send,
{
    fn as_str(&self) -> &str {
        self.1.as_str()
    }
}
