//! Contains types for the [`RaidData`] structure and its respective structures.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::serialization;

use super::{Effect, Released, ID};

/// Contains data including **[`Raids`][`Raid`]** and other kinds of information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RaidData {
    #[serde(alias = "Raid")]
    pub raids: Vec<Raid>,
    // pub raid_seasons: Vec<RaidSeason>,
    // pub time_attack: Vec<TimeAttack>,
    // pub time_attack_rules: Vec<TimeAttackRule>,
    pub world_raid: Vec<Raid>,
}

/// **A Blue Archive raid.**
///
/// Has information related to a specific raid, including icons, release status and stats.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Raid {
    pub id: ID,
    /// Whether a `raid` has been **[`Released`]** in a specific region or not.
    #[serde(alias = "IsReleased")]
    pub released: Released,
    pub max_difficulty: Option<Vec<u8>>,
    pub path_name: String,
    pub faction: Option<Faction>,
    pub terrain: Vec<String>,
    pub bullet_type: String,
    pub bullet_type_insane: Option<String>,
    armor_type: String,
    pub enemy_list: Vec<Vec<u32>>, // todo: might associate with ID's of enemies. Will need further lookup in order to deserialize.
    #[serde(alias = "RaidSkill")]
    pub skills: Vec<Skill>,
    pub exclude_normal_attack: Option<Vec<u32>>,
    pub name: String,
    pub icon: Option<String>,
    #[serde(alias = "IconBG")]
    pub icon_bg: Option<String>,
    pub difficulty_name: Option<Vec<String>>,
    pub difficulty_max: Option<Vec<i8>>,
    pub profile: Option<String>,
    #[serde(alias = "WorldBossHP")]
    pub world_boss_hp: Option<u64>,
    pub level: Option<Vec<i32>>,
}

impl Raid {
    pub fn armor(&self) -> crate::Armor {
        crate::Armor::from_str(&self.armor_type)
            .unwrap_or(crate::Armor::Unknown(self.armor_type.clone()))
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq,)]
// #[serde(rename_all = "PascalCase")]
// pub struct RaidSeason;

// #[derive(Debug, Serialize, Deserialize, PartialEq,)]
// #[serde(rename_all = "PascalCase")]
// pub struct TimeAttack;

// #[derive(Debug, Serialize, Deserialize, PartialEq,)]
// #[serde(rename_all = "PascalCase")]
// pub struct TimeAttackRule;

/// **A [`Raid`] specific Skill**.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "SkillType", rename_all = "PascalCase")]
pub enum Skill {
    #[serde(alias = "normal")]
    Normal {
        #[serde(alias = "Id")]
        id: String,
        min_difficulty: Option<u8>,
        #[serde(alias = "ATGCost")]
        atg_cost: u8,
        icon: Option<String>,
        name: Option<String>,
        #[serde(
            alias = "Desc",
            deserialize_with = "serialization::deserialize_html_encoded_string"
        )]
        description: String,
    },
    #[serde(alias = "raidautoattack")]
    RaidAutoAttack {
        #[serde(alias = "Id")]
        id: String,
        min_difficulty: Option<u8>,
        #[serde(alias = "ATGCost")]
        atg_cost: u8,
        icon: Option<String>,
        effects: Option<Vec<Effect>>,
    },
    EX(SpecialRaidSkill),
    Passive(SpecialRaidSkill),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialRaidSkill {
    #[serde(alias = "Id")]
    id: String,
    min_difficulty: Option<u8>,
    #[serde(alias = "ATGCost")]
    atg_cost: u8,
    icon: String,
    effects: Option<Vec<Effect>>,
    name: String,
    #[serde(
        alias = "Desc",
        deserialize_with = "serialization::deserialize_html_encoded_string"
    )]
    pub description: String,
    parameters: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Display, Deserialize, Serialize, PartialEq, Clone, EnumString)]
pub enum Faction {
    Decagrammaton,
    Slumpia,
    #[strum(to_string = "Communio Sanctorum")]
    CommunioSanctorum,
    Kaitenger,
    #[strum(to_string = "The Library of Lore")]
    TheLibraryofLore,
    #[strum(to_string = "Seven Prisoners")]
    SevenPrisoners,
    #[strum(to_string = "Hundred Ghost Tales")]
    HundredGhostTales,
}
