use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    enums::{Club, Rarity, School, StudentFilter},
    Armor, Damage, Position, Role, Squad, Weapon,
};

/**
    A `struct` when a [`Student`] is searched with an ID.
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IDStudent {
    /// The ID of the student.
    pub id: u32,
    /// TBD
    pub localize_etc_id: u32,
    /// The name of the student.
    pub name: String,
    /// When the student was released.
    pub released: bool,
    /// If the student is playable.
    pub playable: bool,
    /// The amount of stars the student has upon obtaining them.
    pub base_star: u8,
    /// The rarity of the student.
    pub rarity: String,
    /// The type of armor the student has.
    pub armor_type: String,
    /// The type of damage (or the type of bullet) of the student.
    pub bullet_type: String,
    /// The position of the student.
    pub position: String,
    /// The role of the student.
    pub role: String,
    /// The type of squad of the student.
    pub squad_type: String,
    /// The type of weapon the student uses.
    pub weapon_type: String,
    /// The club the student is apart of.
    pub club: String,
    /// The school the student is apart of.
    pub school: String,
    /// TBD
    pub equipment_type: Vec<String>,
    /// TBD
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
    /// The ID of the student.
    pub id: u32,
    /// The amount of stars the student has upon obtaining them.
    pub base_star: u8,
    /// The rarity of the student.
    pub rarity: String,
    /// The name of the student.
    pub name: String,
    /// The profile of the student.
    pub profile: String,
    /// The type of armor of the student.
    pub armor_type: String,
    /// The type of damage (or the type of bullet) of the student.
    pub bullet_type: String,
    /// The position of the student.
    pub position: String,
    /// The role of the student.
    pub role: String,
    /// The type of squad of the student.
    pub squad_type: String,
    /// The type of weapon the student has.
    pub weapon_type: String,
    /// The type of terrain modifiers.
    pub terrain: Terrain,
}

/**
    The "desired" information of a Blue Archive student, and contains the most data.

    Has special methods to access certain data easier.
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    /// The ID associated with the student.
    pub id: u32,
    /// If the student is released.
    pub is_released: bool,
    /// If the student is a playable character in the game.
    pub is_playable: bool,
    /// Details about the character, mainly relating to their name and profile.
    pub character: Character,
    /// Details about the character pertaining to their age, school, and other things.
    pub info: Info,
    /// Details pertaining to the stats of the student.
    #[serde(alias = "stat")]
    pub stats: Stats,
    /// Details relating to the terrain modifiers of the student.
    pub terrain: Terrain,
    /// Details relating to the skills of the student.
    pub skills: Skills,
}

impl Student {
    /// The name of the [`Student`].
    pub fn name(&self) -> String {
        self.character.name.to_string()
    }

    /// The [`StudentID`] of the [`Student`].
    pub fn id(&self) -> StudentID {
        StudentID(self.id)
    }

    /// Whether the [`Student`] was released or not.
    pub fn released(&self) -> Released {
        Released(self.is_released)
    }

    /// The profile of the [`Student`].
    pub fn profile(&self) -> String {
        self.character.profile.to_string()
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

    /// The [`Rarity`] of the [`Student`].
    pub fn rarity(&self) -> Rarity {
        self.character.rarity.clone()
    }

    /// The [`Squad`] the [`Student`] is apart of.
    pub fn squad(&self) -> Squad {
        match Squad::from_str(self.character.squad_type.as_str()) {
            Ok(squad) => squad,
            Err(_) => Squad::Unknown(self.character.squad_type.to_string()),
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
        match School::from_str(&self.info.school) {
            Ok(school) => school,
            Err(_) => School::Unknown(self.info.school.clone()),
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

    /// The [`Position`] the [`Student`] belongs to.
    pub fn position(&self) -> Position {
        match Position::from_str(&self.character.position) {
            Ok(pos) => pos,
            Err(_) => Position::Unknown(self.character.position.clone()),
        }
    }

    /// The type of [`Damage`] the [`Student`] deals.
    pub fn damage(&self) -> Damage {
        match Damage::from_str(&self.character.bullet_type) {
            Ok(damage) => damage,
            Err(_) => Damage::Unknown(self.character.bullet_type.clone()),
        }
    }

    /// The type of [`Armor`] the [`Student`] has.
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
            "(#{} :: {} :: Age: {} :: School:{})",
            self.id,
            self.name(),
            self.age(),
            self.school()
        )
    }
}

/// The ID of a Blue Archive Student.
#[derive(Debug, Eq, PartialEq)]
pub struct StudentID(pub u32);

impl StudentFilter for StudentID {
    fn filter<'student>(&self, students: &'student [Student]) -> Vec<&'student Student> {
        students
            .iter()
            .filter(|student| &student.id() == self)
            .collect()
    }
}

/// The Age of a Blue Archive Student.
///
/// The actual [`Option<u8>`] is wrapped under this struct to make it easier to display.
#[derive(Debug, PartialEq, Eq)]
pub struct Age(pub Option<u8>);

impl Age {
    /// A method to represent [`Age`] as a [`u8`].
    ///
    /// Will return `0` if there is no age for the [`Student`].
    pub fn as_u8(&self) -> u8 {
        self.0.unwrap_or(0)
    }
}

impl From<u8> for Age {
    fn from(value: u8) -> Self {
        Self(Some(value))
    }
}

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(age) => write!(f, "{age}"),
            None => write!(f, "None"),
        }
    }
}

impl StudentFilter for Age {
    fn filter<'student>(&self, students: &'student [Student]) -> Vec<&'student Student> {
        students
            .iter()
            .filter(|student| &student.age() == self)
            .collect()
    }
}

/// Whether the [`Student`] was released or not.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Released(pub bool);

impl StudentFilter for Released {
    fn filter<'student>(&self, students: &'student [Student]) -> Vec<&'student Student> {
        students
            .iter()
            .filter(|student| &student.released() == self)
            .collect()
    }
}
impl std::fmt::Display for Released {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => write!(f, "yes"),
            false => write!(f, "no"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    /// The type of armor the student has (you can also access this through the given method in [`Student`]).
    pub armor_type: String,
    /// The amount of stars a student has upon obtaining them.
    pub base_star: u8,
    /// The type of damage (or type of bullet) of the student (you can also access this through the given method in [`Student`]).
    pub bullet_type: String,
    /// The name of the student (you can also access this through the given method in [`Student`]).
    pub name: String,
    /// The position of the student (you can also access this through the given method in [`Student`]).
    pub position: String,
    /// The profile of the student (you can also access this through the given method in [`Student`]).
    pub profile: String,
    /// The raw rarity of the student (you can also access this through the given method in [`Student`]).
    pub rarity: Rarity,
    /// The role of the student (you can also access this through the given method in [`Student`]).
    pub role: String,
    /// The type of squad of the student (you can also access this through the given method in [`Student`]).
    pub squad_type: String,
    /// The type of weapon the student has (you can also access this through the given method in [`Student`]).
    pub weapon_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The age of the student (represented as ... years old). (you can also access this through the given method in [`Student`]).
    pub age: String,
    /// The artist of the Student.
    #[serde(alias = "artis")]
    pub artist: Option<String>,
    /// The raw club of the student (you can also access this through the given method in [`Student`]).
    pub club: String,
    /// The raw school of the student (you can also access this through the given method in [`Student`]).
    pub school: String,
    /// The given school year of the student.
    pub school_year: String,
    /// The voice actor of the student.
    pub voice_actor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Terrain {
    pub urban: TerrainModifier,
    pub outdoor: TerrainModifier,
    pub indoor: TerrainModifier,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TerrainModifier {
    damage_dealt: String,
    shield_block_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Skills {
    ex: Vec<Skill>,
    normal: Vec<Skill>,
    passive: Vec<Skill>,
    sub: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    level: u32,
    name: String,
    description: String,
    skill_cost: u32,
    bullet_type: String,
}
