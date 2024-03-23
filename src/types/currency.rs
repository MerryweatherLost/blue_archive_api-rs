use serde::{Deserialize, Serialize};

use super::ID;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Currency {
    #[serde(alias = "Id")]
    pub id: ID,
    pub category: String,
    // rarity: String (omitted for now...)
    pub icon: String,
    pub name: String,
    #[serde(alias = "Desc")]
    pub description: String,
}
