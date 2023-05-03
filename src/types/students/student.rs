use std::str::FromStr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{Armor, Position, School, Squad, TacticRole},
    types::{Age, Released, ID},
    BlueArchiveError, IMAGE_DATA_URI,
};

use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    pub id: ID,
    is_released: (bool, bool),
    default_order: u32,
    path_name: String,
    dev_name: String,
    name: String,
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
    street_battle_adaptation: u8,
    outdoor_battle_adaptation: u8,
    indoor_battle_adaptation: u8,
    weapon_type: String,
    weapon_img: String,
    cover: bool,
    equipment: Vec<String>,
    #[serde(alias = "CollectionBG")]
    collection_bg: String,
    collection_texture: String,
    family_name: String,
    family_name_ruby: Option<String>,
    personal_name: String,
    school_year: Option<String>,
    character_age: String,
    birthday: String,
    #[serde(alias = "CharacterSSRNew")]
    character_ssr_new: Option<String>,
    profile_introduction: String,
    hobby: String,
    character_voice: String,
    birth_day: String,
    illustrator: String,
    designer: String,
    char_height_metric: String,
    char_height_imperial: Option<String>,
    stability_point: u32,
    attack_power_1: u32,
    attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    max_hp_100: u32,
    defense_power_1: u32,
    defense_power_100: u32,
    heal_power_1: u32,
    heal_power_100: u32,
    dodge_point: u32,
    accuracy_point: u32,
    critical_point: u32,
    critical_damage_rate: u32,
    ammo_count: u16,
    ammo_cost: u16,
    range: u16,
    regen_cost: u16,
    // skills: Vec<Skill>
    favor_stat_type: Vec<String>,
    favor_stat_value: Vec<Vec<u8>>,
    favor_alts: Vec<u32>,
    memory_lobby: Vec<u8>,
    #[serde(alias = "MemoryLobbyBGM")]
    memory_lobby_bgm: String,
    furniture_interaction: Vec<Vec<u32>>,
    favor_item_tags: Vec<String>,
    favor_item_unique_tags: Vec<String>,
    is_limited: u8,
    weapon: Weapon,
    // todo: Figure out how I can represent Gear {} as None.
    // gear: Gear,
    skill_ex_material: Vec<Vec<u16>>,
    skill_ex_material_amount: Vec<Vec<u8>>,
    skill_material: Vec<Vec<u16>>,
    skill_material_amount: Vec<Vec<u8>>,
    /// Image data related to the [`Student`].
    #[serde(skip)]
    pub image: StudentImageData,
}

impl Student {
    /// The name of the [`Student`].
    pub fn first_name(&self) -> String {
        self.personal_name.clone()
    }

    /// The last name (surname or family name) of the [`Student`].
    pub fn last_name(&self) -> String {
        self.family_name.clone()
    }

    /// Gets the full name of a [`Student`], with the family name (surname) coming first.
    pub fn full_name_with_last(&self) -> String {
        format!("{} {}", self.family_name, self.personal_name)
    }

    /// Gets the full name of a [`Student`], with the personal name coming first.
    pub fn full_name_with_first(&self) -> String {
        format!("{} {}", self.personal_name, self.family_name)
    }

    /// Gets the age of the [`Student`].
    pub fn age(&self) -> Age {
        for id in [" ", "歳", "세", " ปี", "歲"] {
            if let Some(ix) = self.character_age.find(id) {
                if let Ok(num) = self.character_age[0..ix].parse::<u8>() {
                    return Age(Some(num));
                }
            }
        }
        Age(None)
    }

    /// Gets the school of the [`Student`].
    pub fn school(&self) -> School {
        match School::from_str(&self.school) {
            Ok(school) => school,
            Err(_) => School::Unknown(self.school.clone()),
        }
    }

    /// Released status of the [`Student`].
    /// Represented in data as (bool, bool)
    pub fn released(&self) -> Released {
        Released {
            japan: self.is_released.0,
            global: self.is_released.1,
        }
    }

    /// TBD
    pub fn tactic_role(&self) -> TacticRole {
        match TacticRole::from_str(&self.tactic_role) {
            Ok(tr) => tr,
            Err(_) => TacticRole::Unknown(self.tactic_role.clone()),
        }
    }

    /// TBD
    pub fn squad(&self) -> Squad {
        match Squad::from_str(&self.squad_type) {
            Ok(s) => s,
            Err(_) => Squad::Unknown(self.squad_type.clone()),
        }
    }

    /// TBD
    pub fn armor(&self) -> Armor {
        match Armor::from_str(&self.armor_type) {
            Ok(a) => a,
            Err(_) => Armor::Unknown(self.armor_type.clone()),
        }
    }

    /// TBD
    pub fn position(&self) -> Position {
        match Position::from_str(&self.armor_type) {
            Ok(p) => p,
            Err(_) => Position::Unknown(self.armor_type.clone()),
        }
    }

    /// Fetches extra data of this [`Student`].
    pub(crate) async fn fetch_extra_data(&mut self, client: &Client) {
        if let Ok(data) = StudentImageData::new(self, client).await {
            self.image = data
        }
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Student : {} :-: ID#:{} | Age:{} | School: {}",
            self.full_name_with_last(),
            self.id,
            self.age(),
            self.school()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct StudentSummon {
    id: u32,
    source_skill: String,
    inherit_caster_stat: Vec<String>,
    inherit_caster_amount: Option<Vec<Vec<u32>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    released: (bool, bool),
    stat_type: Vec<String>,
    stat_value: Vec<Vec<u8>>,
    name: String,
    desc: String,
    icon: String,
    tier_up_material: Vec<Vec<u16>>,
    tier_up_material_amount: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Weapon {
    name: String,
    desc: String,
    adaptation_type: String,
    adaptation_value: u8,
    attack_power_1: u32,
    attack_power_100: u32,
    #[serde(alias = "MaxHP1")]
    max_hp_1: u32,
    #[serde(alias = "MaxHP100")]
    max_hp_100: u32,
    heal_power_1: u32,
    heal_power_100: u32,
    stat_level_up_type: String, // todo: Coerce to enum.
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StudentImageData {
    /// If there is a portrait associated with this [`Student`].
    pub portrait_url: Option<String>,
    /// If there is an alternative portrait associated with this [`Student`].
    pub alternative_portrait_url: Option<String>,
}

impl StudentImageData {
    pub async fn new(
        student: &Student,
        client: &Client,
    ) -> Result<StudentImageData, BlueArchiveError> {
        Ok(Self {
            portrait_url: Self::fetch_image_with_url(
                client,
                format!(
                    "{IMAGE_DATA_URI}/student/portrait/Portrait_{}.webp",
                    student.dev_name
                ),
            )
            .await,
            alternative_portrait_url: Self::fetch_image_with_url(
                client,
                format!(
                    "{IMAGE_DATA_URI}/student/portrait/Portrait_{}_2.webp",
                    student.dev_name
                ),
            )
            .await,
        })
    }

    async fn fetch_image_with_url(client: &Client, url: impl Into<String>) -> Option<String> {
        let url: String = url.into();
        match client.get(&url).send().await {
            Ok(response) => match response.error_for_status() {
                Ok(_) => Some(url),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
