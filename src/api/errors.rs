#[derive(Debug)]
pub enum BlueArchiveError {
    /// Something goes wrong with requesting data from the API.
    RequestError(reqwest::Error),
    /// In the case that deserialization failed.
    DeserializationError(reqwest::Error),
    /// When randomiation fails.
    RandomError,
}

impl std::fmt::Display for BlueArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlueArchiveError::RequestError(err) => write!(f, "{}", err),
            BlueArchiveError::DeserializationError(err) => write!(f, "{}", err),
            BlueArchiveError::RandomError => write!(
                f,
                "Randomizing students failed due to the slice being empty."
            ),
        }
    }
}

impl std::error::Error for BlueArchiveError {}
