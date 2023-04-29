pub mod student;
use std::fmt::Display;

pub use student::Student;

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
