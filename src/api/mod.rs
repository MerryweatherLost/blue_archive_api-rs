pub mod student;
pub mod summon;

use crate::enums::Language;
use crate::filter::student::StudentFilterOptions;
use crate::types::Student;
use crate::{BlueArchiveError, DATA_URI};

use rand::seq::IteratorRandom;
use reqwest::Client;
pub use reqwest::{Request, Response, StatusCode};

use anyhow::Result;
use strum_macros::Display;

/// Internal functions to work with the data easier.
pub(crate) mod internal {
    use super::*;

    /// Contains the endpoints for the data, they mainly just represent the path of what data is obtained.
    #[derive(Debug, Display)]
    pub(crate) enum Endpoint {
        Currency,
        Enemies,
        Equipment,
        Furniture,
        Items,
        Localization,
        Raids,
        Students,
        Summons,
    }

    /// Fetches a response using a given endpoint.
    pub(crate) async fn fetch_response(
        endpoint: &Endpoint,
        language: &Language,
    ) -> Result<Response, BlueArchiveError> {
        let url = format!(
            "{}/{}/{}.json",
            DATA_URI,
            language.id(),
            endpoint.to_string().to_lowercase()
        );
        Ok(reqwest::get(url).await?.error_for_status()?)
    }
}
