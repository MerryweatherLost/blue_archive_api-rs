use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    enums::{Club, School},
    Armor, Damage, Position, Role, SquadType, Weapon,
};

/**
    A `struct` when a [`Student`] is searched with an ID.
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
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
Contains partial information of a [`Student`], and contains limited data.
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    The "desired" information of a Blue Archive student, and contains the most data.

    Has special methods to access certain data easier.
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// The Age of a Blue Archive Student.
///
/// The actual [`Option<u8>`] is wrapped under this struct to make it easier to display.
#[derive(Debug)]
pub struct Age(Option<u8>);

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(age) => write!(f, "{age}"),
            None => write!(f, "None"),
        }
    }
}

impl Student {
    /// The name of the Student.
    pub fn name(&self) -> String {
        self.character.name.to_string()
    }

    /// The age of the [`Student`], referred to as [`Age<u8>`].
    /// * Unknown Ages or "Top Secret" will be referred to as [`None`].
    pub fn age(&self) -> Age {
        match self.info.age.find(|c| c == ' ') {
            Some(ix) => match self.info.age[0..ix].parse::<u8>() {
                Ok(num) => Age(Some(num)),
                Err(_) => Age(None),
            },
            None => Age(None),
        }
    }

    /// The [`SquadType`] the [`Student`] is apart of.
    pub fn squad_type(&self) -> SquadType {
        match SquadType::from_str(self.character.squad_type.as_str()) {
            Ok(squad) => squad,
            Err(_) => SquadType::Unknown(self.character.squad_type.to_string()),
        }
    }

    /// The [`Weapon`] the [`Student`] uses.
    pub fn weapon(&self) -> Weapon {
        match Weapon::from_str(self.character.weapon_type.as_str()) {
            Ok(weapon) => weapon,
            Err(_) => Weapon::Unknown(self.character.weapon_type.to_string()),
        }
    }

    /// The [`School`] the [`Student`] belongs to.
    pub fn school(&self) -> School {
        match self.info.school.as_str() {
            "Abydos" => School::Abydos,
            "Gehenna" => School::Gehenna,
            "Hyakkiyako" => School::Hyakkiyako,
            "Millennium" => School::Millennium,
            "Shanhaijing" => School::Shanhaijing,
            "Trinity" => School::Trinity,
            _ => School::Unknown(self.info.school.clone()),
        }
    }

    /// The [`Club`] the [`Student`] belongs to.
    pub fn club(&self) -> Club {
        match Club::from_str(&self.info.club) {
            Ok(club) => club,
            Err(_) => Club::Unknown(self.info.club.clone()),
        }
    }

    /// The [`Role`] the [`Student`] is apart of.
    pub fn role(&self) -> Role {
        match Role::from_str(&self.character.role) {
            Ok(role) => role,
            Err(_) => Role::Unknown(self.character.role.clone()),
        }
    }

    pub fn position(&self) -> Position {
        match Position::from_str(&self.character.position) {
            Ok(pos) => pos,
            Err(_) => Position::Unknown(self.character.position.clone()),
        }
    }

    pub fn damage(&self) -> Damage {
        match Damage::from_str(&self.character.bullet_type) {
            Ok(damage) => damage,
            Err(_) => Damage::Unknown(self.character.bullet_type.clone()),
        }
    }

    pub fn armor(&self) -> Armor {
        match Armor::from_str(&self.character.armor_type) {
            Ok(armor) => armor,
            Err(_) => Armor::Unknown(self.character.armor_type.clone()),
        }
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, Age: {}, apart of {}",
            self.name(),
            self.age(),
            self.school()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub age: String,
    #[serde(alias = "artis")]
    pub artist: Option<String>,
    pub club: String,
    pub school: String,
    pub school_year: String,
    pub voice_actor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Terrain {
    pub urban: TerrainModifier,
    pub outdoor: TerrainModifier,
    pub indoor: TerrainModifier,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TerrainModifier {
    damage_dealt: String,
    shield_block_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Skills {
    ex: Vec<Skill>,
    normal: Vec<Skill>,
    passive: Vec<Skill>,
    sub: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    level: u32,
    name: String,
    description: String,
    skill_cost: u32,
    bullet_type: String,
}
