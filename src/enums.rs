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
    **This is a `enum` that contains the current Blue Archive weapons represented in the API.**

    As of the `27th of November, 2022`,
    this is the current list of weapons represented in the API.
    * **AR** (Assault Rifle)
    * **GL** (Grenade Launcher)
    * **HG** (Handgun)
    * **MG** (Machine Gun)
    * **MT** (Mortar)
    * **RF** (Rifle)
    * **RG** (Railgun)
    * **RL** (Rocket Launcher)
    * **SG** (Shotgun)
    * **SMG** (Submachine Gun)
    * **SR** (Sniper RIfle)

    In the case that a weapon in the API is not present on the wrapper,
    a [`Weapon::Unknown(String)`] is returned to represent the unknown weapon with its name in the `enum`.
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

impl Weapon {
    pub fn full_name(&self) -> String {
        match self {
            Weapon::AR => "Assault Rifle".to_string(),
            Weapon::GL => "Grenade Launcher".to_string(),
            Weapon::HG => "Handgun".to_string(),
            Weapon::MG => "Machine Gun".to_string(),
            Weapon::MT => "Mortar".to_string(),
            Weapon::RF => "Rifle".to_string(),
            Weapon::RG => "Railgun".to_string(),
            Weapon::RL => "Rocket Launcher".to_string(),
            Weapon::SG => "Shotgun".to_string(),
            Weapon::SMG => "Submachine Gun".to_string(),
            Weapon::SR => "Sniper Rifle".to_string(),
            Weapon::Unknown(string) => string.to_string(),
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::AR => write!(f, "AR"),
            Weapon::GL => write!(f, "GL"),
            Weapon::HG => write!(f, "HG"),
            Weapon::MG => write!(f, "MG"),
            Weapon::MT => write!(f, "MT"),
            Weapon::RF => write!(f, "RF"),
            Weapon::RG => write!(f, "RG"),
            Weapon::RL => write!(f, "RL"),
            Weapon::SG => write!(f, "SG"),
            Weapon::SMG => write!(f, "SMG"),
            Weapon::SR => write!(f, "SR"),
            Weapon::Unknown(string) => write!(f, "{string}"),
        }
    }
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
