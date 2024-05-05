//! Contains types for [`Students`][`Student`].

pub mod student;
use std::fmt::Display;

use serde::ser::SerializeStruct;

use serde::{Deserialize, Serialize};
pub use student::Student;

/// The age of a **[`Student`]**, which can be **[`None`]** or a **[`u8`]**, depending on if the age can be parsed or not.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Age(pub Option<u8>);
impl Age {
    /// Returns the underlying value, though if [`None`], it will return `0`.
    pub fn as_u8(&self) -> u8 {
        self.0.unwrap_or(0)
    }
}

impl From<u8> for Age {
    fn from(value: u8) -> Self {
        Self(Some(value))
    }
}

impl Serialize for Age {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(v) => serializer.serialize_some::<u8>(&v),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let radix = 10;
        let mut num_sequence: Vec<u8> = vec![];
        for char in String::deserialize(deserializer)?.chars() {
            match char.to_digit(radix.into()) {
                Some(digit) => num_sequence.push(digit as u8),
                None => break,
            }
        }
        Ok(Age((!num_sequence.is_empty()).then_some(
            num_sequence.iter().fold(0, |acc, el| acc * radix + el),
        )))
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(age) => write!(f, "{}", age),
            None => write!(f, "None"),
        }
    }
}

/// The released status of a **[`Student`]**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Released {
    pub japan: bool,
    pub global: bool,
    pub china: bool,
}

impl Serialize for Released {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut released = serializer.serialize_struct("IsReleased", 3)?;
        released.serialize_field("japan", &self.japan)?;
        released.serialize_field("global", &self.global)?;
        released.serialize_field("china", &self.china)?;
        released.end()
    }
}

impl<'de> Deserialize<'de> for Released {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::deserialize(deserializer)?;
        Ok(Self {
            japan: vec[0],
            global: vec[1],
            china: vec[2],
        })
    }
}

impl Display for Released {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(JP: {}, GL: {}, CN, {})",
            self.japan, self.global, self.china
        )
    }
}

/// The height of a student, represented in a [`metric`](`Height::metric`) or [`imperial`](`Height::imperial`) standard.
#[derive(Debug, PartialEq, Eq)]
pub struct Height {
    /// The student height in metric standard.
    pub metric: String,
    /// The student height in imperial standard. Due to ambiguity of the structure, it may be missing from the data.
    pub imperial: Option<String>,
}
impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Metric: {}, Imperial: {:?})",
            self.metric, self.imperial
        )
    }
}
