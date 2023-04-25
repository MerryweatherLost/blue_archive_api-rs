use serde::{Deserialize, Serialize};

use crate::{BlueArchiveError, IMAGE_DATA_URI};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    pub id: u32,
    is_released: Vec<bool>,
    default_order: u32, // <- ?
    path_name: String,
    dev_name: String,
    name: String,
    school: String,
    club: String,
    star_grade: u8,
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
    family_name_ruby: String,
    personal_name: String,
    school_year: String,
    character_age: String,
    birthday: String,
    #[serde(alias = "CharacterSSRNew")]
    character_ssr_new: String,
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

    pub fn image(&self) -> StudentImageData {
        StudentImageData { student: self }
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} : {} : {})",
            self.full_name_with_last(),
            self.character_age,
            self.school
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    released: Vec<bool>,
    stat_type: Vec<String>,
    stat_value: Vec<Vec<u8>>,
    name: String,
    desc: String,
    icon: String,
    tier_up_material: Vec<Vec<u16>>,
    tier_up_material_amount: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct StudentImageData<'student> {
    student: &'student Student,
}

impl StudentImageData<'_> {
    /// If there is a portrait associated with this [`Student`].
    pub async fn portrait(&self) -> Result<Option<String>, BlueArchiveError> {
        let alt_portrait_url = format!(
            "{IMAGE_DATA_URI}/student/portrait/Portrait_{}.webp",
            self.student.dev_name
        );
        match reqwest::get(&alt_portrait_url).await?.error_for_status() {
            Ok(_) => Ok(Some(alt_portrait_url)),
            Err(_) => Ok(None),
        }
    }

    /// If there is an alternative portrait associated with this [`Student`].
    pub async fn alternative_portrait(&self) -> Result<Option<String>, BlueArchiveError> {
        let alt_portrait_url = format!(
            "{IMAGE_DATA_URI}/student/portrait/Portrait_{}_2.webp",
            self.student.dev_name
        );
        match reqwest::get(&alt_portrait_url).await?.error_for_status() {
            Ok(_) => Ok(Some(alt_portrait_url)),
            Err(_) => Ok(None),
        }
    }
}
