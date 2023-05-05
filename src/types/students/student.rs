//! Contains the [`Student`] structure and its respective structures.

use std::str::FromStr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    enums::*,
    types::{Age, Released, ID},
    BlueArchiveError, IMAGE_DATA_URI,
};

use anyhow::Result;

use super::Height;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    /// The **[`ID`]** of the student.
    pub id: ID,
    is_released: (bool, bool),
    default_order: u32,
    path_name: String,
    dev_name: String,
    /// The name of the student, which is their last, or family name.
    /// This is just a shorthand to it.
    pub name: String,
    school: String,
    club: String,
    /// The amount of stars a [`Student`] is rated.
    #[serde(alias = "StarGrade")]
    stars: u8,
    squad_type: String,
    tactic_role: String,
    summons: Vec<StudentSummon>,
    position: String,
    bullet_type: String,
    armor_type: String,
    pub street_battle_adaptation: u8,  // todo
    pub outdoor_battle_adaptation: u8, // todo
    pub indoor_battle_adaptation: u8,  // todo
    weapon_type: String,
    weapon_img: String,
    pub cover: bool,            // todo
    pub equipment: Vec<String>, // todo
    #[serde(alias = "CollectionBG")]
    collection_bg: String,
    collection_texture: String,
    family_name: String,
    family_name_ruby: Option<String>,
    personal_name: String,
    pub school_year: Option<String>,
    character_age: String,
    /// The birthday of the student represented as (Month, Day).
    pub birthday: String,
    #[serde(alias = "CharacterSSRNew")]
    character_ssr_new: Option<String>,
    /// Also known as the profile of the student. Provides a brief explanation of their background.
    #[serde(alias = "ProfileIntroduction")]
    pub description: String,
    /// The hobby of the student.
    pub hobby: String,
    /// The voice actor of the student.
    #[serde(alias = "CharacterVoice")]
    pub voice_actor: String,
    /// The birthday of the student represented as (MM/DD).
    #[serde(alias = "BirthDay")]
    pub birthday_short: String,
    /// The illustrator of the art of this student.
    pub illustrator: String,
    /// The designer of this student, often related to the [`illustrator`](Student.illustrator) field.
    pub designer: String,
    char_height_metric: String,
    char_height_imperial: Option<String>,
    pub stability_point: u32,
    pub attack_power_1: u32,
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    pub defense_power_1: u32,
    pub defense_power_100: u32,
    pub heal_power_1: u32,
    pub heal_power_100: u32,
    pub dodge_point: u32,
    pub accuracy_point: u32,
    pub critical_point: u32,
    pub critical_damage_rate: u32,
    pub ammo_count: u16,
    pub ammo_cost: u16,
    pub range: u16,
    pub regen_cost: u16,
    /// Contains a collection of **[`Skills`][`Skill`]**.
    pub skills: Vec<Skill>,
    pub favor_stat_type: Vec<String>,   // todo
    pub favor_stat_value: Vec<Vec<u8>>, // todo
    pub favor_alts: Vec<u32>,           // todo
    pub memory_lobby: Vec<u8>,          // todo
    /// The name of the music in the students' recollection lobby.
    #[serde(alias = "MemoryLobbyBGM")]
    pub memory_lobby_bgm: String,
    pub furniture_interaction: Vec<Vec<u32>>,   // todo
    pub favor_item_tags: Vec<String>,           // todo
    pub favor_item_unique_tags: Vec<String>,    // todo
    pub is_limited: u8, // todo: represent this as enum. Limited::(0?, 1?, 2?)
    pub weapon: Weapon, // todo
    pub gear: GearKind, // todo
    pub skill_ex_material: Vec<Vec<u16>>, // todo
    pub skill_ex_material_amount: Vec<Vec<u8>>, // todo
    pub skill_material: Vec<Vec<u16>>, // todo
    pub skill_material_amount: Vec<Vec<u8>>, // todo
    /// Image data related to the [`Student`].
    #[serde(skip)]
    pub image: StudentImageData,
}

impl Student {
    /// The **first name (`personal_name`)** of the student.
    pub fn first_name(&self) -> String {
        self.personal_name.clone()
    }

    /// The **last name/surname (`family_name`)** of the student.
    pub fn last_name(&self) -> String {
        self.family_name.clone()
    }

    /// Gets the full name of a student, with the **surname (`family_name`)** coming first.
    pub fn full_name_last(&self) -> String {
        format!("{} {}", self.family_name, self.personal_name)
    }

    /// Gets the full name of a student, with the **first name (`personal_name`)** coming first.
    pub fn full_name_first(&self) -> String {
        format!("{} {}", self.personal_name, self.family_name)
    }

    /// Gets the **[`Age`]** of the student.
    pub fn age(&self) -> Age {
        let radix = 10_u8;
        let mut num_sequence: Vec<u8> = vec![];
        for char in self.character_age.chars() {
            match char.to_digit(radix.into()) {
                Some(digit) => num_sequence.push(digit as u8),
                None => break,
            }
        }
        Age((!num_sequence.is_empty())
            .then_some(num_sequence.iter().fold(0, |acc, el| acc * radix + el)))
    }

    /// The **[`Released`]** status of the student.
    pub fn released(&self) -> Released {
        Released {
            japan: self.is_released.0,
            global: self.is_released.1,
        }
    }

    /// Gets the [`Height`] of the [`Student`].
    pub fn height(&self) -> Height {
        Height {
            metric: self.char_height_metric.clone(),
            imperial: self.char_height_imperial.clone(),
        }
    }

    /// Tries to get a **[`Gear`]** from data.
    pub fn gear(&self) -> Option<Gear> {
        self.gear.get().cloned()
    }

    /// Gets the **[`School`]** of the student.
    pub fn school(&self) -> School {
        School::from_str(&self.school).unwrap_or_else(|_| School::Unknown(self.school.clone()))
    }

    /// Gets the **[`TacticalRole`]** of the student.
    pub fn tactical_role(&self) -> TacticalRole {
        TacticalRole::from_str(&self.tactic_role)
            .unwrap_or_else(|_| TacticalRole::Unknown(self.tactic_role.clone()))
    }

    /// Gets the **[`Squad`]** of the student.
    pub fn squad(&self) -> Squad {
        Squad::from_str(&self.squad_type)
            .unwrap_or_else(|_| Squad::Unknown(self.squad_type.clone()))
    }

    /// Gets the **[`Armor`]** of the student.
    pub fn armor(&self) -> Armor {
        Armor::from_str(&self.armor_type)
            .unwrap_or_else(|_| Armor::Unknown(self.armor_type.clone()))
    }

    /// Gets the **[`Position`]** of the student.
    pub fn position(&self) -> Position {
        Position::from_str(&self.armor_type)
            .unwrap_or_else(|_| Position::Unknown(self.armor_type.clone()))
    }

    /// Gets the **[`BulletType`]** of the student.
    pub fn bullet_type(&self) -> BulletType {
        BulletType::from_str(&self.bullet_type)
            .unwrap_or_else(|_| BulletType::Unknown(self.bullet_type.clone()))
    }

    /// Gets the **[`Club`]** of the student.
    pub fn club(&self) -> Club {
        Club::from_str(&self.club).unwrap_or_else(|_| Club::Unknown(self.club.clone()))
    }

    /// Gets the **[`WeaponType`]** of the student.
    pub fn weapon_type(&self) -> WeaponType {
        WeaponType::from_str(&self.weapon_type)
            .unwrap_or_else(|_| WeaponType::Unknown(self.weapon_type.clone()))
    }

    /// Fetches extra data of this **[`Student`]**.
    ///
    /// Has a possibility of failing when trying to use the [`Client`].
    pub(crate) async fn fetch_extra_data(
        &mut self,
        client: &Client,
    ) -> Result<(), BlueArchiveError> {
        self.image = StudentImageData::new(self, client).await?;
        Ok(())
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Student : {} :-: {} | Age:{} | School: {}",
            self.full_name_last(),
            self.id,
            self.age(),
            self.school()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StudentSummon {
    id: u32,
    source_skill: String,
    inherit_caster_stat: Vec<String>,
    inherit_caster_amount: Option<Vec<Vec<u32>>>,
}

/// The kind of [`Gear`] that the data may represent.
///
/// There is an issue where Gear in data is represented as `Gear {}`, therefore this is a mitigation against that.
/// If you have a better implementation of handling this, as in allowing for me to represent the data as an `Option<Gear>`, please send a PR.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum GearKind {
    Present(Gear),
    Empty(EmptyGear),
}
impl GearKind {
    /// Attempts to get a **[`Gear`]**, though if it gets an [`GearKind::Empty`], it will return [`None`].
    pub const fn get(&self) -> Option<&Gear> {
        match self {
            Self::Present(gear) => Some(gear),
            Self::Empty(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    released: (bool, bool),
    stat_type: Vec<String>,
    stat_value: Vec<Vec<u16>>,
    name: String,
    desc: String,
    icon: String,
    tier_up_material: Vec<Vec<u16>>,
    tier_up_material_amount: Vec<Vec<u8>>,
}

/// There is an issue where Gear in data is represented as `Gear {}`, therefore this is a mitigation against that.
/// If you have a better implementation of handling this, as in allowing for me to represent the data as an `Option<Gear>`, please send a PR.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EmptyGear {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Weapon {
    /// The name of the weapon.
    pub name: String,
    /// The description of the weapon.
    #[serde(alias = "Desc")]
    pub description: String,
    adaptation_type: String,
    pub adaptation_value: u8,
    pub attack_power_1: u32,
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    pub heal_power_1: u32,
    pub heal_power_100: u32,
    stat_level_up_type: String, // todo: Coerce to enum.
}

/// A **[`Student`]** skill.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    skill_type: String,
    parameters: Option<Vec<Vec<String>>>,
    cost: Option<Vec<u32>>,
    icon: Option<String>,
    effects: Vec<Effect>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Effect {
    #[serde(alias = "Type")]
    kind: String,
    stat: Option<String>,
    hits: Option<Vec<i32>>,
    scale: Option<Vec<i32>>,
    frames: Option<Frames>,
    critical_check: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Frames {
    attack_enter_duration: u8,
    attack_start_duration: u8,
    attack_end_duration: u8,
    attack_burst_round_over_delay: u8,
    #[serde(alias = "AttackIngDuration")]
    attacking_duration: u8,
    attack_reload_duration: u8,
}

/// Image data related to a **[`Student`]**.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct StudentImageData {
    /// The portrait associated with this **[`Student`]**.
    pub portrait: Portrait,
    /// The **[`Weapon`]** icon url belonging to the **[`Student`]**.
    pub weapon_icon: String,
}

impl StudentImageData {
    /// Creates itself from a given **[`Student`]** and **[`reqwest::Client`]**.
    pub async fn new(student: &Student, client: &Client) -> Result<Self, BlueArchiveError> {
        Ok(Self {
            portrait: Portrait {
                full_body_url: format!(
                    "{IMAGE_DATA_URI}/student/portrait/Portrait_{}.webp",
                    student.dev_name
                ),
                icon_url: format!(
                    "{IMAGE_DATA_URI}/student/icon/{}.png",
                    student.collection_texture
                ),
                alternative_full_body_url: Self::fetch_image_with_url(
                    client,
                    format!(
                        "{IMAGE_DATA_URI}/student/portrait/Portrait_{}_2.webp",
                        student.dev_name
                    ),
                )
                .await,
                bg: format!("{IMAGE_DATA_URI}/background/{}.jpg", student.collection_bg),
            },
            weapon_icon: format!("{IMAGE_DATA_URI}/weapon/{}.png", student.weapon_img),
        })
    }

    async fn fetch_image_with_url(client: &Client, url: impl Into<String>) -> Option<String> {
        let url: String = url.into();
        (client.get(&url).send().await).map_or(None, |response| match response.error_for_status() {
            Ok(_) => Some(url),
            Err(_) => None,
        })
    }
}

/// Contains portrait data of a **[`Student`]**.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Portrait {
    /// The full body image url associated with this **[`Student`]**.
    pub full_body_url: String,
    /// The icon url associated with this **[`Student`]**.
    pub icon_url: String,
    /// If there is an alternative full-body image url associated with this **[`Student`]**.
    pub alternative_full_body_url: Option<String>,
    /// The background image url associated with this **[`Student`]**.
    pub bg: String,
}
