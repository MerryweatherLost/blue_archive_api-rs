//! Contains many structures that make up the deserialized data.

pub mod raids;
pub mod students;
pub mod summons;

pub use raids::RaidData;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
pub use students::{Age, Released, Student};
pub use summons::Summon;

/// **A Blue Archive ID**.
///
/// Basically wraps around a [`u32`], and exists for representation of an identifier that can be filtered and have extra functionality.
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Type")]
pub enum Effect {
    Accumulation {
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    BuffSelf {
        #[serde(alias = "Stat")]
        stat: String,
        #[serde(alias = "StackSame")]
        stack_same: Option<u8>,
        #[serde(alias = "Value")]
        value: Option<Vec<Vec<i32>>>,
        #[serde(alias = "Scale")]
        scale: Option<Vec<i32>>,
        #[serde(alias = "Channel")]
        channel: Option<i32>,
        #[serde(alias = "Icon")]
        icon: Option<String>,
    },
    BuffTarget {
        #[serde(alias = "Restrictions")]
        restrictions: Option<Vec<Restriction>>,
        #[serde(alias = "Value")]
        value: Vec<Vec<i32>>,
        #[serde(alias = "Stat")]
        stat: String,
        #[serde(alias = "Channel")]
        channel: i32,
    },
    DMGSingle {
        #[serde(alias = "SourceStat")]
        source_stat: Option<String>,
        #[serde(alias = "Critical")]
        critical: Option<i8>,
        #[serde(alias = "CriticalCheck")]
        critical_check: Option<CriticalCheck>,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
        #[serde(alias = "IgnoreDef")]
        ignore_def: Option<Vec<i32>>,
        #[serde(alias = "Hits")]
        hits: Option<Vec<i32>>,
        #[serde(alias = "Frames")]
        frames: Option<Frames>,
    },
    DMGMulti {
        #[serde(alias = "Critical")]
        critical: Option<i8>,
        #[serde(alias = "CriticalCheck")]
        critical_check: Option<CriticalCheck>,
        #[serde(alias = "SubstituteCondition")]
        substitute_condition: Option<String>,
        #[serde(alias = "Hits")]
        hits: Vec<i32>,
        #[serde(alias = "HitsParameter")]
        hits_parameter: Option<i8>,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
        #[serde(alias = "SubstituteScale")]
        substitute_scale: Option<Vec<i32>>,
        #[serde(alias = "IgnoreDef")]
        ignore_def: Option<Vec<i32>>,
        #[serde(alias = "Frames")]
        frames: Option<Frames>,
    },
    DMGEcho {
        #[serde(alias = "CriticalCheck")]
        critical_check: CriticalCheck,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
        #[serde(alias = "IgnoreDef")]
        ignore_def: Option<Vec<i32>>,
    },
    DMGDot {
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            alias = "Duration"
        )]
        duration: u32,
        #[serde(deserialize_with = "deserialize_number_from_string", alias = "Period")]
        period: u32,
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    DMGZone {
        #[serde(alias = "ZoneHitInterval")]
        zone_hit_interval: Option<i32>,
        #[serde(alias = "ZoneDuration")]
        zone_duration: Option<i32>,
        #[serde(alias = "HitFrames")]
        hit_frames: Option<Vec<u16>>,
        #[serde(alias = "CriticalCheck")]
        critical_check: CriticalCheck,
        #[serde(alias = "Hits")]
        hits: Option<Vec<i32>>,
        #[serde(alias = "HitsParameter")]
        hits_parameter: Option<i8>,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    DMGByHit {
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    DMGEchoWithScaling {
        #[serde(alias = "CriticalCheck")]
        critical_check: CriticalCheck,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    HealDot {
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            alias = "Duration"
        )]
        duration: u32,
        #[serde(deserialize_with = "deserialize_number_from_string", alias = "Period")]
        period: u32,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    Heal {
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    HealZone {
        #[serde(alias = "HitFrames")]
        hit_frames: Vec<i32>,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    CrowdControl {
        #[serde(deserialize_with = "deserialize_number_from_string", alias = "Chance")]
        chance: u32,
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    BuffAlly {
        #[serde(alias = "Restrictions")]
        restrictions: Option<Vec<Restriction>>,
        #[serde(alias = "Value")]
        value: Vec<Vec<i32>>,
        #[serde(alias = "Stat")]
        stat: String,
        #[serde(alias = "Channel")]
        channel: i32,
    },
    Shield {
        #[serde(alias = "Scale")]
        scale: ScaleValue,
    },
    FormChange {
        #[serde(alias = "HideFormChangeIcon")]
        hide_form_change_icon: Option<bool>,
        #[serde(alias = "Frames")]
        frames: Frames,
        #[serde(alias = "Hits")]
        hits: Vec<i32>,
        #[serde(alias = "CriticalCheck")]
        critical_check: Option<CriticalCheck>,
        #[serde(alias = "Scale")]
        scale: Option<Vec<i32>>,
    },
    IgnoreDelay {
        #[serde(alias = "Scale")]
        scale: Vec<u8>,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ScaleValue {
    D1(Vec<i32>),
    D2(Vec<Vec<i32>>),
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Restriction {
    pub property: String,
    pub operand: String,
    /// A restriction value which can contain a **[`i32`]** or **[`String`]**.
    pub value: RestrictValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RestrictValue {
    String(String),
    I32(i32),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CriticalCheck {
    Check,
    Always,
    #[serde(other)]
    Unknown,
}
