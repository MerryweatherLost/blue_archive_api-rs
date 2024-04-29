//! This is where the main asynchronous (and if elligble, blocking) API implementation is.
//! You are able to obtain data about multiple entities in the game here.

#[cfg(feature = "blocking")]
pub mod blocking;

pub mod currency;
pub mod enemy;
pub mod equipment;
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
        _Furniture,
        _Items,
        _Localization,
        Enemies,
        Equipment,
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

    #[cfg(feature = "blocking")]
    pub(crate) fn get_response(
        endpoint: &Endpoint,
        language: &Language,
        client: &reqwest::blocking::Client,
    ) -> Result<reqwest::blocking::Response, BlueArchiveError> {
        let url = format!(
            "{}/{}/{}.json",
            DATA_URI,
            language.id(),
            endpoint.to_string().to_lowercase()
        );
        Ok(client.get(url).send()?.error_for_status()?)
    }
}
