use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Contains the different kinds of [`Banner`] gachas possible.
///
/// - `Pickup` PickupGacha
/// - `Limited` LimitedGacha
/// - `Fes` FesGacha
#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum Gacha {
    /// Pickup Recruitment Gacha.
    #[serde(alias = "PickupGacha")]
    Pickup,
    /// Limited Gacha.
    #[serde(alias = "LimitedGacha")]
    Limited,
    /// Unsure of what this is exactly, "Festival?"
    #[serde(alias = "FesGacha")]
    Fes,
}

/// Banners data of all Blue Archive [`Banner`]'s.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banners {
    pub current: Vec<Banner>,
    pub upcoming: Vec<Banner>,
    pub ended: Vec<Banner>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub gacha_type: Gacha,
    start_at: i64,
    end_at: i64,
    /// Contains the names of the students who have higher rates of being picked.
    #[serde(alias = "rateups")]
    pub rateup_student_names: Vec<String>,
}

impl Banner {
    /// When the banner starts (or started).
    pub fn starts(&self) -> Option<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::from_timestamp_millis(self.start_at)
    }

    /// When the banner ends (or ended).
    pub fn ends(&self) -> Option<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::from_timestamp_millis(self.end_at)
    }
}

impl std::fmt::Display for Banner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<type:({}), starts:({:?}) : ends:({:?}), rateup_student_names:({:?})>",
            self.gacha_type,
            self.starts(),
            self.ends(),
            self.rateup_student_names
        )
    }
}
