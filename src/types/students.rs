use serde::{Deserialize, Serialize};

use crate::enums;

/**
    A `struct` when a [`Student`] is searched with an ID.
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IDStudent {
    pub id: u32,
    pub localize_etc_id: u32,
    pub name: String,
    pub released: bool,
    pub playable: bool,
    pub base_star: u8,
    pub rarity: String,
    pub armor_type: String,
    pub bullet_type: String,
    pub position: String,
    pub role: String,
    pub squad_type: String,
    pub weapon_type: String,
    pub club: String,
    pub school: String,
    pub equipment_type: Vec<String>,
    pub tags: Vec<String>,
}

/**
    Underlying data as result of the response given that contains a [`Vec<PartialStudent>`].
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialStudentData {
    pub data: Vec<PartialStudent>,
}

/**
Contains partial information of a [`Student`]. Contains limited data.
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialStudent {
    pub id: u32,
    pub base_star: u8,
    pub rarity: String,
    pub name: String,
    pub profile: String,
    pub armor_type: String,
    pub bullet_type: String,
    pub position: String,
    pub role: String,
    pub squad_type: String,
    pub weapon_type: String,
    pub terrain: Terrain,
}

/**
    The "desired" information of a Blue Archive student. Contains the most data.
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub id: u32,
    pub is_released: bool,
    pub is_playable: bool,
    pub character: Character,
    pub info: Info,
    #[serde(alias = "stat")]
    pub stats: Stats,
    pub terrain: Terrain,
    pub skills: Skills,
}

impl Student {
    pub fn school(&self) -> enums::School {
        match self.info.school.as_str() {
            "Abydos" => enums::School::Abydos,
            "Gehenna" => enums::School::Gehenna,
            "Hyakkiyako" => enums::School::Hyakkiyako,
            "Millennium" => enums::School::Millennium,
            "Shanhaijing" => enums::School::Shanhaijing,
            "Trinity" => enums::School::Trinity,
            _ => enums::School::Unknown(self.info.school.clone()),
        }
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.character.name, self.info.age)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub armor_type: String,
    pub base_star: u32,
    pub bullet_type: String,
    pub name: String,
    pub position: String,
    pub profile: String,
    pub rarity: String,
    pub role: String,
    pub squad_type: String,
    pub weapon_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub age: String,
    #[serde(alias = "artis")]
    pub artist: String,
    pub club: String,
    pub school: String,
    pub school_year: String,
    pub voice_actor: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub id: u32,
    pub attack_level_1: u32,
    pub attack_level_100: u32,
    #[serde(alias = "maxHPLevel1")]
    pub max_hp_level_1: u32,
    #[serde(alias = "maxHPLevel100")]
    pub max_hp_level_100: u32,
    pub defense_level_1: u32,
    pub defense_level_100: u32,
    pub heal_power_level_1: u32,
    pub heal_power_level_100: u32,
    pub def_penetrate_level_1: u32,
    pub def_penetrate_level_100: u32,
    pub ammo_count: u32,
    pub ammo_cost: u32,
    pub range: u32,
    pub move_speed: u32,
    pub street_mood: String,
    pub outdoor_mood: String,
    pub indoor_mood: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terrain {
    pub urban: TerrainModifier,
    pub outdoor: TerrainModifier,
    pub indoor: TerrainModifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerrainModifier {
    damage_dealt: String,
    shield_block_rate: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skills {
    ex: Vec<Skill>,
    normal: Vec<Skill>,
    passive: Vec<Skill>,
    sub: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    level: u32,
    name: String,
    description: String,
    skill_cost: u32,
    bullet_type: String,
}
