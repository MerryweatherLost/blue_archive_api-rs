//! Error handling for the api wrapper.

use thiserror::Error;

/// Contains underlying information on why an error has happened with the wrapper.
/// For now, it internally holds a **[`reqwest::Error`]** `struct`.
#[derive(Debug, Error)]
pub struct BlueArchiveError(#[from] pub reqwest::Error);

impl std::fmt::Display for BlueArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
