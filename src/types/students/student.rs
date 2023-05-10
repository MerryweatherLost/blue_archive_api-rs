//! Contains the [`Student`] structure and its respective structures.

use std::str::FromStr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    enums::*,
    types::{Age, Released, Skill, ID},
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
    pub default_order: u32,
    pub path_name: String,
    pub dev_name: String,
    /// The name of the student as presented in the data, and can have an associated tag alongside it.
    /// An example would be **`Toki (Bunny)`**.
    pub name: String,
    school: String,
    club: String,
    /// The amount of stars a [`Student`] is rated.
    #[serde(alias = "StarGrade")]
    pub stars: u8,
    squad_type: String,
    tactic_role: String,
    pub summons: Vec<StudentSummon>,
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
    profile_introduction: String,
    hobby: String,
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
    pub furniture_interaction: Vec<Vec<u32>>, // todo
    pub favor_item_tags: Vec<String>,         // todo
    pub favor_item_unique_tags: Vec<String>,  // todo
    pub is_limited: u8,                       // todo: represent this as enum. Limited::(0?, 1?, 2?)
    pub weapon: Weapon,
    gear: GearKind,
    pub skill_ex_material: Vec<Vec<u16>>,       // todo
    pub skill_ex_material_amount: Vec<Vec<u8>>, // todo
    pub skill_material: Vec<Vec<u16>>,          // todo
    pub skill_material_amount: Vec<Vec<u8>>,    // todo
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

    /// Also known as the **profile** of the student. Provides a brief explanation of their background.
    pub fn description(&self) -> String {
        html_escape::decode_html_entities(&self.profile_introduction).into()
    }

    /// The quote said when obtaining this student (if an SSR).
    pub fn quote_ssr(&self) -> Option<String> {
        self.character_ssr_new.as_ref().and_then(|quote| {
            (!quote.is_empty()).then_some(html_escape::decode_html_entities(&quote).into())
        })
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
            imperial: self
                .char_height_imperial
                .as_ref()
                .map(|height| html_escape::decode_html_entities(&height).to_string()),
        }
    }

    /// The hobby of the student if they have one.
    pub fn hobby(&self) -> Option<String> {
        (self.hobby != "None").then_some(self.hobby.clone())
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
    pub id: ID,
    pub source_skill: String,
    pub inherit_caster_stat: Vec<String>,
    pub inherit_caster_amount: Option<Vec<Vec<u32>>>,
}

/// The kind of [`Gear`] that the data may represent.
///
/// There is an issue where Gear in data is represented as `"gear": {}`, therefore this is a mitigation against that.
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
    pub stat_type: Vec<String>,
    pub stat_value: Vec<Vec<u16>>,
    pub name: String,
    #[serde(alias = "Desc")]
    pub description: String,
    icon: String,
    pub tier_up_material: Vec<Vec<u16>>,
    pub tier_up_material_amount: Vec<Vec<u8>>,
}
impl Gear {
    /// Whether a specific gear was **[released][Released]** or not in a specific region.
    pub fn released(&self) -> Released {
        Released {
            japan: self.released.0,
            global: self.released.1,
        }
    }

    /// Returns the url of a gear icon.
    pub fn icon_url(&self) -> String {
        format!("{IMAGE_DATA_URI}/gear/{}", self.icon)
    }
}
/// There is an issue where Gear in data is represented as `"gear": {}`, therefore this is a mitigation against that.
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
    pub adaptation_type: String,
    pub adaptation_value: u8,
    pub attack_power_1: u32,
    pub attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    pub max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    pub max_hp_100: u32,
    pub heal_power_1: u32,
    pub heal_power_100: u32,
    pub stat_level_up_type: LevelUpType,
}

/// The level-up type of a **[`Weapon`]**.
#[derive(Debug, strum_macros::Display, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LevelUpType {
    Standard,
    Premature,
    LateBloom,
    #[serde(other)]
    Unknown,
}

/// Image data related to a **[`Student`]**.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct StudentImageData {
    /// The portrait associated with this **[`Student`]**.
    pub portrait: Portrait,
    /// The **[`Weapon`]** icon url belonging to the **[`Student`]**.
    pub weapon_icon_url: String,
}

impl StudentImageData {
    /// Creates itself from a given **[`Student`]** and **[`reqwest::Client`]**.
    ///
    /// Will query for extra image data when constructed.
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
                bg_url: format!("{IMAGE_DATA_URI}/background/{}.jpg", student.collection_bg),
            },
            weapon_icon_url: format!("{IMAGE_DATA_URI}/weapon/{}.png", student.weapon_img),
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
    pub bg_url: String,
}
