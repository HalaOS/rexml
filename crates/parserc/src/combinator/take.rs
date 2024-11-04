use crate::{inputs::InputStream, Parser};

#[allow(unused)]
struct TakeWhileWith<I, F> {
    input: I,
    cond: F,
}

/// Returns the longest input slice (if any) that matches the predicate.
pub fn take_while_with<I, F>(cond: F, input: I) -> TakeWhileWith<I, F>
where
    I: InputStream,
{
    TakeWhileWith { input, cond }
}
