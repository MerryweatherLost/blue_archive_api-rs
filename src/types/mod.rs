//! Contains many structures that make up the deserialized data.

pub mod raids;
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
    /// An id that has so far been associated with `raids`.
    pub id: Option<String>,
    #[serde(alias = "SkillType")]
    pub kind: SkillKind,
    pub min_difficulty: Option<u8>,
    #[serde(alias = "ATGCost")]
    /// So far normally associated with `raids`.
    pub atg_cost: Option<u8>,
    /// So far normally associated with `raids`.
    name: Option<String>,
    desc: Option<String>,
    pub parameters: Option<Vec<Vec<String>>>,
    pub cost: Option<Vec<u32>>,
    pub icon: Option<String>,
    pub show_info: Option<bool>,
    pub effects: Vec<Effect>,
}
impl Skill {
    /// The name of the skill.
    pub fn name(&self) -> Option<String> {
        self.name
            .as_ref()
            .map(|value| html_escape::decode_html_entities(&value).into())
    }

    /// The description of the skill.
    pub fn description(&self) -> Option<String> {
        self.desc
            .as_ref()
            .map(|value| html_escape::decode_html_entities(&value).into())
    }
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
#[serde(tag = "Type")]
pub enum Effect {
    Accumulation {
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
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
        scale: Vec<i32>,
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
        scale: Vec<i32>,
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
        scale: Vec<i32>,
        #[serde(alias = "IgnoreDef")]
        ignore_def: Option<Vec<i32>>,
    },
    DMGDot {
        #[serde(alias = "Duration")]
        duration: String, // todo: parse.
        #[serde(alias = "Period")]
        period: String, // todo: parse.
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
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
        hits: Vec<i32>,
        #[serde(alias = "HitsParameter")]
        hits_parameter: i8,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    DMGByHit {
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    DMGEchoWithScaling {
        #[serde(alias = "CriticalCheck")]
        critical_check: CriticalCheck,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    HealDot {
        #[serde(alias = "Duration")]
        duration: String, // todo: parse.
        #[serde(alias = "Period")]
        period: String, // todo: parse.
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    Heal {
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    HealZone {
        #[serde(alias = "HitFrames")]
        hit_frames: Vec<i32>,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
    },
    CrowdControl {
        #[serde(alias = "Chance")]
        chance: String, // todo: parse.
        #[serde(alias = "Icon")]
        icon: String,
        #[serde(alias = "Scale")]
        scale: Vec<i32>,
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
        scale: Vec<i32>,
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Restriction {
    pub property: String,
    pub operand: String,
    /// A restriction value which can contain a **[`i32`]** or **[`String`]**.
    pub value: RestrictValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum RestrictValue {
    String(String),
    I32(i32),
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
