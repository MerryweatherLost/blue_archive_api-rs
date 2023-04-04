use thiserror::Error;

/// Errors for the Blue Archive API.
///
/// Contains a set of enum values for more context on what goes wrong.
#[derive(Debug, Error)]
pub enum BlueArchiveError {
    #[error("requesting data failed. ({0})")]
    Reqwest(#[from] reqwest::Error),
}
