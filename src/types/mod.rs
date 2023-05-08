//! Contains many structures that make up the deserialized data.

pub mod students;
pub mod summons;

use serde::{Deserialize, Serialize};
pub use students::{Age, Released, Student};
pub use summons::Summon;

/// **A Blue Archive ID**.
///
/// Basically wraps around a [`u32`], and exists for representation of an identifier that can be filtered and have extra functionality.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ID(pub u32);

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID#:({})", self.0)
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("ID", &self.0)
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: u32 = Deserialize::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

/// **A Blue Archive Skill**.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    #[serde(alias = "SkillType")]
    pub kind: SkillKind,
    pub parameters: Option<Vec<Vec<String>>>,
    pub cost: Option<Vec<u32>>,
    pub icon: Option<String>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum SkillKind {
    #[serde(alias = "weaponpassive")]
    WeaponPassive,
    #[serde(alias = "sub")]
    Sub,
    #[serde(alias = "ex")]
    Ex,
    #[serde(alias = "normal")]
    Normal,
    #[serde(alias = "autoattack")]
    AutoAttack,
    #[serde(alias = "passive")]
    Passive,
    #[serde(alias = "gearnormal")]
    GearNormal,
    #[serde(other)]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Effect {
    #[serde(alias = "Type")]
    pub kind: EffectKind,
    pub stat: Option<String>,
    pub hits: Option<Vec<i32>>,
    pub scale: Option<Vec<i32>>,
    pub frames: Option<Frames>,
    pub critical_check: Option<CriticalCheck>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EffectKind {
    Accumulation,
    DMGSingle,
    DMGMulti,
    DMGEcho,
    DMGDot,
    DMGZone,
    DMGByHit,
    DMGEchoWithScaling,
    BuffSelf,
    HealDot,
    Heal,
    HealZone,
    CrowdControl,
    BuffTarget,
    BuffAlly,
    Shield,
    FormChange,
    IgnoreDelay,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Frames {
    pub attack_enter_duration: u8,
    pub attack_start_duration: u8,
    pub attack_end_duration: u8,
    pub attack_burst_round_over_delay: u8,
    #[serde(alias = "AttackIngDuration")]
    pub attacking_duration: u8,
    pub attack_reload_duration: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum CriticalCheck {
    Check,
    Always,
    #[serde(other)]
    Unknown,
}
