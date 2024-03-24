//! Contains types for the [`Summon`] structure and its respective structures.

use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::{serialization, Armor, BulletType, WeaponType, IMAGE_DATA_URI};

use super::{Effect, Radius, ID};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Summon {
    pub id: ID,
    pub skills: Vec<Skill>,
    pub name: String,
    pub dev_name: String,
    #[serde(alias = "Type")]
    pub kind: String,
    pub tactic_role: Option<String>,
    pub star_bonus: Option<bool>,
    bullet_type: String,
    armor_type: String,
    pub street_battle_adaptation: Option<u8>,
    pub outdoor_battle_adaptation: Option<u8>,
    pub indoor_battle_adaptation: Option<u8>,
    weapon_type: Option<String>,
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
    pub ammo_count: u32,
    pub ammo_cost: u8,
    pub range: u16,
    pub move_speed: u16,
    pub regen_cost: u16,
}
impl Summon {
    /// Gets the **[`Bullet`]** type of the summon.
    pub fn bullet_type(&self) -> BulletType {
        BulletType::from_str(&self.bullet_type)
            .unwrap_or(BulletType::Unknown(self.bullet_type.clone()))
    }

    /// Gets the **[`Armor`]** of the summon.
    pub fn armor(&self) -> Armor {
        Armor::from_str(&self.armor_type).unwrap_or(Armor::Unknown(self.armor_type.clone()))
    }

    /// Gets the **[`WeaponType`] of the summon.
    pub fn weapon_type(&self) -> WeaponType {
        match &self.weapon_type {
            Some(weapon_type) => WeaponType::from_str(weapon_type)
                .unwrap_or(WeaponType::Unknown(weapon_type.clone())),
            None => WeaponType::None,
        }
    }
}

/// **[`Summon`] specific Skills**.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "SkillType")]
pub enum Skill {
    #[serde(alias = "autoattack")]
    AutoAttack {
        effects: Option<Vec<Effect>>,
        radius: Option<Vec<Radius>>,
    },
    #[serde(alias = "normal")]
    Normal(NormalSkill),
    #[serde(alias = "passive")]
    Passive(PassiveSkill),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NormalSkill {
    pub name: String,
    /** The description of a normal skill. */
    #[serde(
        alias = "Desc",
        deserialize_with = "serialization::deserialize_html_encoded_string"
    )]
    pub description: String,
    parameters: Vec<Vec<String>>,
    pub duration: Option<u32>,
    pub range: Option<u32>,
    pub radius: Option<Vec<Radius>>,
    icon: String,
    is_summon_skill: bool,
    pub effects: Option<Vec<Effect>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PassiveSkill {
    pub name: String,
    /** The description of a passive skill. */
    #[serde(
        alias = "Desc",
        deserialize_with = "serialization::deserialize_html_encoded_string"
    )]
    pub description: String,
    parameters: Vec<Vec<String>>,
    pub radius: Option<Vec<Radius>>,
    icon: String,
    pub is_summon_skill: bool,
    pub effects: Option<Vec<Effect>>,
}

impl NormalSkill {
    /** Gets the icon of this skill represented in a `URI`. */
    pub fn icon(&self) -> String {
        format!("{IMAGE_DATA_URI}/skill/{}.webp", self.icon)
    }
}

impl PassiveSkill {
    /** Gets the icon of this skill represented in a `URI`. */
    pub fn icon(&self) -> String {
        format!("{IMAGE_DATA_URI}/skill/{}.webp", self.icon)
    }
}
