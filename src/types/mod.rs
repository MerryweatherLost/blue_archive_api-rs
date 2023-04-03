//! The types associated with the API.

use serde::{Deserialize, Serialize};

pub mod banners;
pub mod equipment;
pub mod raids;
pub mod students;

pub use banners::*;
pub use equipment::*;
pub use raids::*;
pub use students::*;

/**
Contains the current status of the API, created by [**torikushiii**](https://github.com/torikushiii)

**Github:** <https://github.com/torikushiii/BlueArchiveAPI>
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct APIStatus {
    /// The status code of the API.
    #[serde(alias = "status")]
    pub code: u16,
    /// The version of the API.
    pub version: String,
    /// How long the API has been up, I'm unsure of the measurement, but it is most likely in **miliseconds**.
    pub uptime: u64,
    /// The list of available endpoints for the API.
    pub endpoints: Vec<String>,
}
