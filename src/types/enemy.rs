use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Armor, BulletType, Squad, WeaponType};

use super::ID;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
    #[serde(alias = "Id")]
    pub id: ID,
    pub dev_name: String,
    pub name: String,
    squad_type: String,
    pub rank: String, // todo; could be enum
    bullet_type: String,
    armor_type: String,
    weapon_type: String,
    pub size: Option<String>,
    pub icon: Option<String>,
    pub stability_point: u32,
    pub stability_rate: Option<u32>,
    #[serde(alias = "AttackPower1")]
    pub attack_power_1: u32,
    #[serde(alias = "AttackPower100")]
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    #[serde(alias = "DefensePower1")]
    pub defense_power_1: u32,
    #[serde(alias = "DefensePower100")]
    pub defense_power_100: u32,
    #[serde(alias = "HealPower1")]
    pub heal_power_1: u32,
    #[serde(alias = "HealPower100")]
    pub heal_power_100: u32,
    pub dodge_point: u32,
    pub accuracy_point: u32,
    pub critical_point: u32,
    pub critical_damage_rate: u32,
    pub critical_resist_point: u32,
    pub critical_damage_resist_rate: u32,
    pub range: u32,
    pub damaged_ratio: u32,
}

impl Enemy {
    /// Gets the **[`Squad`]** of the enemy.
    pub fn squad(&self) -> Squad {
        Squad::from_str(&self.squad_type).unwrap_or(Squad::Unknown(self.squad_type.clone()))
    }

    /// Gets the **[`Armor`]** of the enemy.
    pub fn armor(&self) -> Armor {
        Armor::from_str(&self.armor_type).unwrap_or(Armor::Unknown(self.armor_type.clone()))
    }

    /// Gets the **[`BulletType`]** of the enemy.
    pub fn bullet_type(&self) -> BulletType {
        BulletType::from_str(&self.bullet_type)
            .unwrap_or(BulletType::Unknown(self.bullet_type.clone()))
    }

    /// Gets the **[`WeaponType`]** of the enemy.
    pub fn weapon_type(&self) -> WeaponType {
        WeaponType::from_str(&self.weapon_type)
            .unwrap_or(WeaponType::Unknown(self.weapon_type.clone()))
    }
}
