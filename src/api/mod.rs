//! The main module where obtaining the data happens.
pub mod currency;
pub mod raid;
pub mod student;
pub mod summon;

use crate::enums::Language;
use crate::filter::student::StudentFilterOptions;
use crate::types::{RaidData, Student, Summon};
use crate::{BlueArchiveError, DATA_URI};

use rand::seq::IteratorRandom;
use reqwest::Client;
pub use reqwest::{Request, Response, StatusCode};

use anyhow::Result;
use strum_macros::Display;

/// Internal functions to work with the data easier.
pub(crate) mod internal {
    use super::{BlueArchiveError, Client, Display, Language, Response, Result, DATA_URI};

    /// Contains the endpoints for the data, they mainly just represent the path of what data is obtained.
    #[derive(Debug, Display)]
    pub enum Endpoint {
        _Enemies,
        _Equipment,
        _Furniture,
        _Items,
        _Localization,
        Currency,
        Raids,
        Students,
        Summons,
    }

    /// Fetches a response using a given endpoint.
    pub(crate) async fn fetch_response(
        endpoint: &Endpoint,
        language: &Language,
        client: &Client,
    ) -> Result<Response, BlueArchiveError> {
        let url = format!(
            "{}/{}/{}.json",
            DATA_URI,
            language.id(),
            endpoint.to_string().to_lowercase()
        );
        Ok(client.get(url).send().await?.error_for_status()?)
    }
}
