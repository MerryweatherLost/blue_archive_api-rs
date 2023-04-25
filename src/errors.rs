use thiserror::Error;

#[derive(Debug, Error)]
pub struct BlueArchiveError(#[from] pub reqwest::Error);

impl std::fmt::Display for BlueArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
