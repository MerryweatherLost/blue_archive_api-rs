//! Contains types for the [`Summon`] structure and its respective structures.

use serde::{Deserialize, Serialize};

use super::ID;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Summon {
    pub id: ID,
    // pub skills: Vec<Skill>,
    pub name: String,
    pub dev_name: String,
    #[serde(alias = "Type")]
    pub kind: String,
    pub tactic_role: Option<String>,
    pub star_bonus: Option<bool>,
    pub bullet_type: String,
    pub armor_type: String,
    pub street_battle_adaptation: Option<u8>,
    pub outdoor_battle_adaptation: Option<u8>,
    pub indoor_battle_adaptation: Option<u8>,
    pub weapon_type: Option<String>,
    pub stability_point: u16,
    pub stability_rate: Option<u16>,
    pub attack_power_1: u16,
    pub attack_power_100: u16,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u16,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u16,
    pub defense_power_1: u16,
    pub defense_power_100: u16,
    pub heal_power_1: u16,
    pub heal_power_100: u16,
    pub dodge_point: u16,
    pub accuracy_point: u16,
    pub critical_point: u16,
    pub critical_damage_rate: u32,
    pub ammo_count: u8,
    pub ammo_cost: u8,
    pub range: u16,
    pub move_speed: u16,
    pub regen_cost: u16,
}
