//! Contains types for [`Students`][`Student`].

pub mod student;
use std::fmt::Display;

pub use student::Student;

/// The age of a **[`Student`]**, which can be **[`None`]** or a **[`u8`]**, depending on if the age can be parsed or not.
#[derive(Debug, PartialEq, Eq)]
pub struct Age(pub Option<u8>);
impl Age {
    pub fn as_u8(&self) -> u8 {
        self.0.unwrap_or(0)
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
#[derive(Debug, PartialEq, Eq)]
pub struct Released {
    pub japan: bool,
    pub global: bool,
}

impl Display for Released {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Japan: {} | Global: {})", self.japan, self.global)
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
        write!(f, "(Metric: {} | Global: {:?})", self.metric, self.imperial)
    }
}
