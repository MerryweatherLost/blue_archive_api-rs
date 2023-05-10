use serde::{Deserialize, Serialize};

use super::{Released, Skill, ID};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Raids {
    pub raid: Vec<Raid>,
    pub world_raid: Vec<Raid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Raid {
    pub id: ID,
    is_released: (bool, bool),
    pub max_difficulty: Vec<u8>,
    pub path_name: String,
    pub faction: String, // todo: -> enum | remove pub when done.
    pub terrain: Vec<String>,
    pub bullet_type: String, // todo: might associate with BulletType enum?
    pub bullet_type_insane: String, // ???
    pub armor_type: String,
    pub enemy_list: Vec<Vec<u32>>, // todo: might associate with ID's of enemies. Will need further lookup in order to deserialize.
    pub raid_skill: Vec<Skill>,
    pub exclude_normal_attack: Vec<u32>,
    pub name: String,
    pub profile: String,
}

impl Raid {
    /// Whether a `raid` has been **[released][`Released`]** in a specific region or not.
    pub fn released(&self) -> Released {
        Released {
            japan: self.is_released.0,
            global: self.is_released.1,
        }
    }
}
