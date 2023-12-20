//! Contains types for the [`RaidData`] structure and its respective structures.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{Effect, Released, SkillKind, ID};

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
    is_released: (bool, bool, bool),
    pub max_difficulty: Option<Vec<u8>>,
    pub path_name: String,
    pub faction: Option<Faction>,
    pub terrain: Vec<String>,
    pub bullet_type: String,
    pub bullet_type_insane: Option<String>,
    armor_type: String,
    pub enemy_list: Vec<Vec<u32>>, // todo: might associate with ID's of enemies. Will need further lookup in order to deserialize.
    pub raid_skill: Vec<Skill>,
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
    /// Whether a `raid` has been **[released][`Released`]** in a specific region or not.
    pub fn released(&self) -> Released {
        Released {
            japan: self.is_released.0,
            global: self.is_released.1,
        }
    }

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
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    pub id: Option<String>,
    #[serde(alias = "SkillType")]
    pub kind: SkillKind,
    pub min_difficulty: Option<u8>,
    #[serde(alias = "ATGCost")]
    pub atg_cost: Option<u8>,
    name: Option<String>,
    desc: Option<String>,
    pub parameters: Option<Vec<Vec<String>>>,
    pub cost: Option<Vec<u32>>,
    pub icon: Option<String>,
    pub show_info: Option<bool>,
    pub effects: Option<Vec<Effect>>,
}
impl Skill {
    /// The name of the skill.
    pub fn name(&self) -> Option<String> {
        self.name
            .as_ref()
            .map(|value| html_escape::decode_html_entities(&value).into())
    }

    /// The description of the skill.
    pub fn description(&self) -> Option<String> {
        self.desc
            .as_ref()
            .map(|value| html_escape::decode_html_entities(&value).into())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Faction {
    Decagrammaton,
    Slumpia,
    CommunioSanctorum,
    Kaitenger,
    TheLibraryofLore,
    SevenPrisoners,
    HundredGhostTales,
}
