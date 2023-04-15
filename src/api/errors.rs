use thiserror::Error;

/// Errors for the Blue Archive API.
///
/// Wraps around a [`reqwest::Error`].
#[derive(Debug, Error)]
#[error(transparent)]
pub struct BlueArchiveError(#[from] pub reqwest::Error);

impl BlueArchiveError {
    /// Returns the [`reqwest::Error`], and consumes itself in the process.
    pub fn reqwest_error(self) -> reqwest::Error {
        self.0
    }

    /// Returns the reference of the [`reqwest::Error`].
    pub fn reqwest_error_ref(&self) -> &reqwest::Error {
        &self.0
    }
}
