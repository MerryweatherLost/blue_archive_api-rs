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
    pub season_id: u16,
    pub boss_name: String,
    pub start_at: String,
    pub end_at: String,
}
