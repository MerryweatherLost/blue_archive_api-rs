use chrono::NaiveDateTime;
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
    pub start_at: i64,
    pub settle_at: i64,
    pub end_at: i64,
}

impl Raid {
    /// When the raid started.
    pub fn start_time(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::from_timestamp_millis(self.start_at)
    }
    /// TBD
    pub fn settle_at_time(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::from_timestamp_millis(self.settle_at)
    }
    /// When the raid ends (or ended at).
    pub fn end_time(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::from_timestamp_millis(self.end_at)
    }
}
