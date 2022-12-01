use anyhow::Result;
use serde::{Deserialize, Serialize};

/**
    Contains all Raid data for Blue Archive.
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Raids {
    pub current: Vec<Raid>,
    pub upcoming: Vec<Raid>,
    pub ended: Vec<Raid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Raid {
    pub season_id: u32,
    pub boss_name: String,
    pub start_at: String,
    pub settle_at: String,
    pub end_at: String,
}

impl Raid {
    /// When the raid started.
    pub fn start_time(&self) -> Result<chrono::NaiveDate, chrono::ParseError> {
        chrono::NaiveDate::parse_from_str(&self.start_at, "%a, %d %b %Y %T %Z")
    }
    /// TBD
    pub fn settle_at_time(&self) -> Result<chrono::NaiveDate, chrono::ParseError> {
        chrono::NaiveDate::parse_from_str(&self.settle_at, "%a, %d %b %Y %T %Z")
    }
    /// When the raid ends (or ended at).
    pub fn end_time(&self) -> Result<chrono::NaiveDate, chrono::ParseError> {
        chrono::NaiveDate::parse_from_str(&self.end_at, "%a, %d %b %Y %T %Z")
    }
}
