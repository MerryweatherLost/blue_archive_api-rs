use serde::{Deserialize, Serialize};

/**
   The `struct` holding data and drops for Equipment.
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    pub data: EquipmentData,
    pub drops: Vec<EquipmentDrop>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentData {
    pub id: u16,
    pub localize_id: Option<u32>,
    pub recipe_id: u16,
    pub category: String,
    pub rarity: String,
    pub max_level: u8,
    pub tier: u8,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDrop {
    pub stage_name: String,
    pub drop_amount: u8,
    pub drop_chance: u16,
}
