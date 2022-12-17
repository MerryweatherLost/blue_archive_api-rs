use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlueArchiveError {
    #[error("requesting data failed.")]
    Reqwest(#[from] reqwest::Error),
    #[error("failed to randomize.")]
    RandomError,
}
