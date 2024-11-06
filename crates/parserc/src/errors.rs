/// Error type raised by this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {}

/// Result type used by this crate.
pub type Result<I, O, E> = std::result::Result<(I, O), (I, E)>;
