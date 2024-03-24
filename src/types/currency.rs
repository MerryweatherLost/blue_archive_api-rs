use serde::{Deserialize, Serialize};

use super::{Rarity, ID};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Currency {
    #[serde(alias = "Id")]
    pub id: ID,
    pub category: String,
    pub rarity: Rarity,
    pub icon: String,
    pub name: String,
    #[serde(alias = "Desc")]
    pub description: String,
}
