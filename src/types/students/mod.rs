//! Contains types for [`Students`][`Student`].

pub mod student;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
pub use student::Student;

/// The age of a **[`Student`]**, which can be **[`None`]** or a **[`u8`]**, depending on if the age can be parsed or not.
#[derive(Debug, PartialEq, Eq)]
pub struct Age(pub Option<u8>);
impl Age {
    pub fn as_u8(&self) -> u8 {
        self.0.unwrap_or(0)
    }
}

impl From<u8> for Age {
    fn from(value: u8) -> Self {
        Self(Some(value))
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(age) => {
                write!(f, "{}", age)
            }
            None => write!(f, "None"),
        }
    }
}

/// The released status of a **[`Student`]**.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Released(bool, bool, bool);

impl Released {
    pub fn japan(&self) -> bool {
        self.0
    }

    pub fn global(&self) -> bool {
        self.1
    }

    pub fn china(&self) -> bool {
        self.2
    }
}

impl Display for Released {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(JP: {} | Global: {})", self.japan(), self.global())
    }
}

/// The height of a student, represented in a [`metric`](`Height.metric`) or [`imperial`](`Height.imperial`) standard.
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
            "(Metric: {} | Imperial: {:?})",
            self.metric, self.imperial
        )
    }
}
