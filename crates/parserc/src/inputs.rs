//! This mod defines input traits for parsers.

use std::{
    future::Future,
    ops::Range,
    str::{CharIndices, Chars},
};

use futures::FutureExt;

/// A [`Next`](InputStream::Next) future implementation whose poll function does nothing but returns a num once.
pub struct NoopNext(pub Option<()>);

impl Future for NoopNext {
    type Output = std::io::Result<Option<()>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(Ok(self.0))
    }
}

/// A [`Next`](InputStream::Next) future returns by [`IntoInputStreams`](IntoInputStreams::next)
pub struct IntoInputStreamsNext<'a, I>
where
    I: IntoIterator,
    I::Item: IntoInputStream,
{
    stream: &'a mut IntoInputStreams<I>,
}

impl<'a, I> Future for IntoInputStreamsNext<'a, I>
where
    I: IntoIterator,
    I::Item: IntoInputStream,
{
    type Output = std::io::Result<Option<()>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {
            if let Some(current) = self.stream.current.as_mut() {
                match current.next().poll_unpin(cx) {
                    std::task::Poll::Ready(Ok(None)) => {}
                    r => return r,
                }
            }

            self.stream.current = self.stream.iter.next().map(|v| v.into_input_stream());

            if self.stream.current.is_none() {
                return std::task::Poll::Ready(Ok(None));
            }
        }
    }
}

/// A future-aware input stream trait for parsers.
pub trait InputStream {
    /// The type of sequence item of this stream.
    type Item;

    /// An iterator over items of the read buffer.
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;

    /// An iterator over items of the read buffer, and their positions.
    type IterIndices<'a>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'a;

    /// A future to process `next` operation.
    type Next<'a>: Future<Output = std::io::Result<Option<()>>> + Unpin + 'a
    where
        Self: 'a;

    /// A slice of inner read buffer.
    type Slice<'a>: AsRef<[u8]>
    where
        Self: 'a;

    /// advance the cursor of read buffer with `count` steps.
    ///
    /// If the count > [`len`](InputStream::len), this function will panic.
    ///
    /// Returns the dropping slice of the read buffer.
    ///
    /// # panic
    ///
    /// Call [`next`](InputStream::next) first, otherwise this function may panic.
    fn advance(&mut self, count: usize) -> Self::Slice<'_>;

    /// Returns an iterator over items of the read buffer.
    ///
    /// # panic
    ///
    /// Call [`next`](InputStream::next) first, otherwise this function may panic.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns an iterator over items of the read buffer, and their positions.
    ///
    /// # panic
    ///
    /// Call [`next`](InputStream::next) first, otherwise this function may panic.
    fn iter_indices(&self) -> Self::IterIndices<'_>;

    /// Create a slice of read buffer.
    ///
    /// # panic
    ///
    /// Call [`next`](InputStream::next) first, otherwise this function may panic.
    fn slice(&self, range: Range<usize>) -> Self::Slice<'_>;

    /// Start a asynchronously read processing.
    ///
    /// This function first clears the read buffer and then reads items from upstream
    /// and stores them into the read buffer.
    fn next(&mut self) -> Self::Next<'_>;
}

impl<T> InputStream for &mut T
where
    T: InputStream,
{
    type Item = T::Item;

    type Iter<'a> = T::Iter<'a>
    where
        Self: 'a;

    type IterIndices<'a> = T::IterIndices<'a>
    where
        Self: 'a;

    type Next<'a> = T::Next<'a>
    where
        Self: 'a;

    type Slice<'a> = T::Slice<'a>
    where
        Self: 'a;

    fn advance(&mut self, count: usize) -> Self::Slice<'_> {
        T::advance(self, count)
    }

    fn iter(&self) -> Self::Iter<'_> {
        T::iter(self)
    }

    fn iter_indices(&self) -> Self::IterIndices<'_> {
        T::iter_indices(self)
    }

    fn slice(&self, range: Range<usize>) -> Self::Slice<'_> {
        T::slice(self, range)
    }

    fn next(&mut self) -> Self::Next<'_> {
        T::next(self)
    }
}

/// Convert a type into [`InputStream`]
pub trait IntoInputStream {
    type InputStream: InputStream;

    /// Convert self into [`InputStream`]
    fn into_input_stream(self) -> Self::InputStream;
}

/// A [`InputStream`] wrapper for `&str` instance.
pub struct InputStreamStr<'a> {
    input: &'a str,
    /// flag for next function.
    once: Option<()>,
}

impl<'a> From<&'a str> for InputStreamStr<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            once: Some(()),
            input: value,
        }
    }
}

impl<'input> InputStream for InputStreamStr<'input> {
    type Item = char;

    type Iter<'a> = Chars<'a>
    where
        Self: 'a;

    type IterIndices<'a> = CharIndices<'a>
    where
        Self: 'a;

    type Next<'a> = NoopNext
    where
        Self: 'a;

    type Slice<'a> = &'input str
    where
        Self: 'a;

    fn advance(&mut self, count: usize) -> Self::Slice<'_> {
        let (first, last) = self.input.split_at(count);
        self.input = last;

        first
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.input.chars()
    }

    fn iter_indices(&self) -> Self::IterIndices<'_> {
        self.input.char_indices()
    }

    fn slice(&self, range: Range<usize>) -> Self::Slice<'_> {
        let start = range.start;
        let end = range.end;

        if start == 0 {
            let (first, _) = self.input.split_at(end);
            first
        } else {
            let (_, last) = self.input.split_at(start);
            let (first, _) = last.split_at(end);

            first
        }
    }

    fn next(&mut self) -> Self::Next<'_> {
        NoopNext(self.once.take())
    }
}

impl<'a> IntoInputStream for &'a str {
    type InputStream = InputStreamStr<'a>;
    fn into_input_stream(self) -> Self::InputStream {
        InputStreamStr::from(self)
    }
}

/// A [`InputStream`] wrapper for `&str` instance.
pub struct InputStreamString {
    input: String,
    offset: usize,
    /// flag for next function.
    once: Option<()>,
}

impl From<String> for InputStreamString {
    fn from(value: String) -> Self {
        Self {
            once: Some(()),
            offset: 0,
            input: value,
        }
    }
}

impl InputStream for InputStreamString {
    type Item = char;

    type Iter<'a> = Chars<'a>
    where
        Self: 'a;

    type IterIndices<'a> = CharIndices<'a>
    where
        Self: 'a;

    type Next<'a> = NoopNext
    where
        Self: 'a;

    type Slice<'a> = &'a str
    where
        Self: 'a;

    fn advance(&mut self, count: usize) -> Self::Slice<'_> {
        self.offset += count;

        let (first, _) = self.input.split_at(self.offset);

        first
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.input[self.offset..].chars()
    }

    fn iter_indices(&self) -> Self::IterIndices<'_> {
        self.input[self.offset..].char_indices()
    }

    fn slice(&self, range: Range<usize>) -> Self::Slice<'_> {
        let start = range.start;
        let end = range.end;

        if start == 0 {
            let (first, _) = self.input.split_at(self.offset + end);
            first
        } else {
            let (_, last) = self.input.split_at(self.offset + start);
            let (first, _) = last.split_at(self.offset + end);

            first
        }
    }

    fn next(&mut self) -> Self::Next<'_> {
        NoopNext(self.once.take())
    }
}

impl IntoInputStream for String {
    type InputStream = InputStreamString;
    fn into_input_stream(self) -> Self::InputStream {
        InputStreamString::from(self)
    }
}

/// A [`InputStream`] wrapper for a sequence of [`IntoInputStream`]s.
pub struct IntoInputStreams<I>
where
    I: IntoIterator,
    I::Item: IntoInputStream,
{
    /// iterator over a sequence of [`IntoInputStream`]s.
    iter: <I as IntoIterator>::IntoIter,

    /// Current processing [`InputStream`].
    current: Option<<I::Item as IntoInputStream>::InputStream>,
}

impl<I> InputStream for IntoInputStreams<I>
where
    I: IntoIterator,
    I::Item: IntoInputStream,
{
    type Item = <<I::Item as IntoInputStream>::InputStream as InputStream>::Item;

    type Iter<'a> = <<I::Item as IntoInputStream>::InputStream as InputStream>::Iter<'a>
    where
        Self: 'a;

    type IterIndices<'a> = <<I::Item as IntoInputStream>::InputStream as InputStream>::IterIndices<'a>
    where
        Self: 'a;

    type Next<'a> = IntoInputStreamsNext<'a, I>
    where
        Self: 'a;

    type Slice<'a> = <<I::Item as IntoInputStream>::InputStream as InputStream>::Slice<'a>
    where
        Self: 'a;

    fn advance(&mut self, count: usize) -> Self::Slice<'_> {
        self.current
            .as_mut()
            .expect("Call 'next' function first")
            .advance(count)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.current
            .as_ref()
            .expect("Call 'next' function first")
            .iter()
    }

    fn iter_indices(&self) -> Self::IterIndices<'_> {
        self.current
            .as_ref()
            .expect("Call 'next' function first")
            .iter_indices()
    }

    fn slice(&self, range: Range<usize>) -> Self::Slice<'_> {
        self.current
            .as_ref()
            .expect("Call 'next' function first")
            .slice(range)
    }

    fn next(&mut self) -> Self::Next<'_> {
        IntoInputStreamsNext { stream: self }
    }
}

impl<T, const N: usize> IntoInputStream for [T; N]
where
    T: IntoInputStream,
{
    type InputStream = IntoInputStreams<std::array::IntoIter<T, N>>;

    fn into_input_stream(self) -> Self::InputStream {
        IntoInputStreams {
            iter: self.into_iter(),
            current: None,
        }
    }
}

impl<T> IntoInputStream for Vec<T>
where
    T: IntoInputStream,
{
    type InputStream = IntoInputStreams<std::vec::IntoIter<T>>;

    fn into_input_stream(self) -> Self::InputStream {
        IntoInputStreams {
            iter: self.into_iter(),
            current: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use quickcheck::quickcheck;

    use super::{InputStream, IntoInputStream};

    async fn test_input_stream<I, F>(mut input: I, f: F)
    where
        I: InputStream,
        F: Fn(&[u8]),
    {
        while let Some(_) = input.next().await.unwrap() {
            let len = input.iter().count();

            let last = input.iter_indices().last().unwrap().0;

            let mut split = 0usize;

            for (index, (offset, _)) in input.iter_indices().enumerate() {
                if len / 2 == index {
                    split = offset;
                }
            }

            f(input.slice(0..split).as_ref());

            f(input.advance(split).as_ref());

            assert_eq!(input.iter().count(), len - len / 2);

            assert_eq!(input.iter_indices().last().unwrap().0, last - split);
        }
    }

    #[test]
    fn test_str() {
        fn prop(input: Vec<String>) {
            block_on(async move {
                test_input_stream("hello world".into_input_stream(), |_| {}).await;

                test_input_stream(
                    [
                        ["hello world", "hello world"],
                        ["hello world1", "hello world2"],
                    ]
                    .into_input_stream(),
                    |_| {},
                )
                .await;

                test_input_stream(&mut "hello world".into_input_stream(), |_| {}).await;

                test_input_stream(
                    input
                        .iter()
                        .map(|v| v.as_str())
                        .filter(|v| v.len() > 0)
                        .collect::<Vec<_>>()
                        .into_input_stream(),
                    |_| {},
                )
                .await;
            })
        }

        quickcheck(prop as fn(Vec<String>));

        fn prop_string(input: Vec<String>) {
            block_on(async move {
                test_input_stream(
                    input
                        .into_iter()
                        .filter(|v| v.len() > 0)
                        .collect::<Vec<_>>()
                        .into_input_stream(),
                    |_| {},
                )
                .await;
            })
        }

        quickcheck(prop_string as fn(Vec<String>));
    }
}
