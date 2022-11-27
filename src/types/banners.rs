use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banners {
    pub current: Vec<Banner>,
    pub upcoming: Vec<Banner>,
    pub ended: Vec<Banner>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub id: u32,
    pub gacha_type: String,
    pub started_at: String, // make not visible, will be turned into a time that is obtained through a function.
    pub ended_at: String, // make not visible, will be turned into a time that is obtained through a function.
    pub rateups: Vec<String>,
}
