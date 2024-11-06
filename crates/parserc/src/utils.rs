/// A utility trait used by combinators.
pub trait FindChar {
    fn find(&self, c: char) -> bool;
}

impl FindChar for &str {
    fn find(&self, c: char) -> bool {
        self.chars().find(|cc| *cc == c).is_some()
    }
}

impl FindChar for String {
    fn find(&self, c: char) -> bool {
        self.chars().find(|cc| *cc == c).is_some()
    }
}

/// A utility trait used by combinators.
pub trait Prefix {
    /// Returns prefix length.
    fn len(&self) -> usize;
    fn find(&self, bytes: &[u8]) -> bool;
}

impl Prefix for &str {
    fn len(&self) -> usize {
        str::len(&self)
    }
    fn find(&self, bytes: &[u8]) -> bool {
        bytes.starts_with(self.as_bytes())
    }
}

impl Prefix for String {
    fn len(&self) -> usize {
        str::len(&self)
    }
    fn find(&self, bytes: &[u8]) -> bool {
        bytes.starts_with(self.as_bytes())
    }
}

impl Prefix for &[u8] {
    fn find(&self, bytes: &[u8]) -> bool {
        bytes.starts_with(self)
    }

    fn len(&self) -> usize {
        <[u8]>::len(&self)
    }
}
