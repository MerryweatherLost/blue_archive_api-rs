use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::serialization;

use super::{Rarity, Released, ID};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Equipment {
    #[serde(alias = "Id")]
    pub id: ID,
    pub name: String,
    #[serde(alias = "IsReleased")]
    pub released: Released,
    #[serde(
        alias = "Desc",
        deserialize_with = "serialization::deserialize_html_encoded_string"
    )]
    pub description: String,
    pub category: EquipmentCategory,
    pub rarity: Rarity,
    pub tier: u8,
    pub icon: String,
    pub shops: Vec<Shop>,
    pub stat_type: Vec<String>,
    pub stat_value: Vec<Vec<u32>>,
    pub recipe: Option<Vec<Vec<u32>>>,
    pub recipe_cost: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Shop {
    #[serde(alias = "ShopCategory")]
    pub category: ShopCategory,
    pub released: Released,
    pub amount: u32,
    pub cost_type: CostType,
    pub cost_id: u16,
    pub cost_amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EquipmentCategory {
    Shoes,
    Hat,
    Watch,
    Badge,
    Bag,
    Charm,
    Necklace,
    Hairpin,
    Gloves,
    Exp,
    WeaponExpGrowthA,
    WeaponExpGrowthC,
    WeaponExpGrowthB,
    WeaponExpGrowthZ,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Display, EnumString)]
pub enum ShopCategory {
    General,
    #[strum(to_string = "Secret Stone Growth")]
    SecretStoneGrowth,
    #[strum(to_string = "Master Coin")]
    MasterCoin,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CostType {
    Currency,
    Item,
}
