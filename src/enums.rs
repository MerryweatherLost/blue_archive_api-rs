/**
    TBD
*/
pub enum Role {
    Attacker,
    Healer,
    Supporter,
    Tanker,
    Unknown(String),
}

/**
    TBD
*/
pub enum Type {
    Special,
    Striker,
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive schools represented in the API.**

    As of the `27th of November, 2022`,
    this is the current list of schools represented in the API.
    * **Abydos** High School
    * **Gehenna** Academy
    * **Hyakkiyako** Alliance Academy
    * **Millennium** Science School
    * **Shanhaijing** Senior Secondary School
    * **Trinity** General School

    In the case that a school in the API is not present on the wrapper,
    a [`School::Unknown(String)`] is returned to represent the unknown school with its name in the `enum`.
*/
pub enum School {
    Abydos,
    Gehenna,
    Hyakkiyako,
    Millennium,
    Shanhaijing,
    Trinity,
    Unknown(String),
}

impl std::fmt::Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            School::Abydos => write!(f, "Abydos"),
            School::Gehenna => write!(f, "Gehenna"),
            School::Hyakkiyako => write!(f, "Hyakkiyako"),
            School::Millennium => write!(f, "Millennium"),
            School::Shanhaijing => write!(f, "Shanhaijing"),
            School::Trinity => write!(f, "Trinity"),
            School::Unknown(unknown_school) => write!(f, "{}", unknown_school),
        }
    }
}

/**
    TBD
*/
pub enum Position {
    Front,
    Middle,
    Back,
    Unknown(String),
}

/**
    TBD
*/
pub enum Weapon {
    AR,
    GL,
    HG,
    MG,
    MT,
    RF,
    RG,
    RL,
    SG,
    SMG,
    SR,
    Unknown(String),
}

/**
    TBD
*/
pub enum Damage {
    Explosion,
    Mystic,
    Penetration,
    Unknown(String),
}

/**
    TBD
*/
pub enum Armor {
    Heavy,
    Light,
    Special,
}
