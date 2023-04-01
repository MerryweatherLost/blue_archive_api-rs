use thiserror::Error;

/// Errors for the Blue Archive API.
///
/// Contains a set of enum values for more context on what goes wrong.
#[derive(Debug, Error)]
pub enum BlueArchiveError {
    #[error("the entity ({query}) was not found (returned with {status_code})")]
    NotFound {
        /// The name inserted by the user.
        query: String,
        /// The status code of the [`reqwest::Response`].
        status_code: reqwest::StatusCode,
    },
    #[error("requesting data failed. ({0})")]
    Reqwest(#[from] reqwest::Error),
}
