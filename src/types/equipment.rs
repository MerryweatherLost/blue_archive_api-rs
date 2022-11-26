use serde::{Deserialize, Serialize};

/**
   The `struct` holding data and drops for Equipment.
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    data: EquipmentData,
    drops: Vec<EquipmentDrop>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentData {
    id: u16,
    localize_id: u32,
    recipe_id: u16,
    category: String,
    rarity: String,
    max_level: u8,
    tier: u8,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentDrop {
    stage_name: String,
    drop_amount: u8,
    drop_chance: u16,
}
