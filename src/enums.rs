use strum_macros::{Display, EnumIter, EnumString};

/**
    **This is a `enum` that contains the current Blue Archive roles represented in the API.**

    As of the `11th of December, 2022`,
    this is the list of current roles represented in the API.
    * **Attacker**
    * **Healer**
    * **Supporter**
    * **Tanker**

    In the case that a role in the API is not present on the wrapper,
    a [`Role::Unknown(String)`] is returned to represent the unknown role with its name in the `enum`.
*/
pub enum Role {
    Attacker,
    Healer,
    Supporter,
    Tanker,
    Unknown(String),
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Attacker => write!(f, "Attacker"),
            Role::Healer => write!(f, "Healer"),
            Role::Supporter => write!(f, "Supporter"),
            Role::Tanker => write!(f, "Tanker"),
            Role::Unknown(unknown_role) => write!(f, "{}", unknown_role),
        }
    }
}

/**
    **This is a `enum` that contains the current Blue Archive weapons represented in the API.**

    As of the `13th of December, 2022`,
    this is the current list of weapons represented in the API.
    * **Special**
    * **Striker**

    In the case that a type in the API is not present on the wrapper,
    a [`Type::Unknown(String)`] is returned to represent the unknown type with its name in the `enum`.
*/
pub enum Type {
    Special,
    Striker,
    Unknown(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Special => write!(f, "Special"),
            Type::Striker => write!(f, "Striker"),
            Type::Unknown(unknown_type) => write!(f, "{}", unknown_type),
        }
    }
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
    **This is a `enum` that contains the current Blue Archive positions represented in the API.**

    As of the `14th of December, 2022`,
    this is the current list of weapons represented in the API.
    * **Front**
    * **Middle**
    * **Back**

    In the case that a weapon in the API is not present on the wrapper,
    a [`Position::Unknown(String)`] is returned to represent the unknown weapon with its name in the `enum`.
*/
pub enum Position {
    Front,
    Middle,
    Back,
    Unknown(String),
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Front => write!(f, "Front"),
            Position::Middle => write!(f, "Middle"),
            Position::Back => write!(f, "Back"),
            Position::Unknown(string) => write!(f, "{string}"),
        }
    }
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
    **This is a `enum` that contains the current Blue Archive damage types represented in the API.**

    As of the `14th of December, 2022`,
    this is the current list of damage types represented in the API.
    * **Explosion**
    * **Mystic**
    * **Penetration**

    In the case that a damage type in the API is not present on the wrapper,
    a [`Damage::Unknown(String)`] is returned to represent the unknown damage type with its name in the `enum`.
*/
pub enum Damage {
    Explosion,
    Mystic,
    Penetration,
    Unknown(String),
}

impl std::fmt::Display for Damage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Damage::Explosion => write!(f, "Explosion"),
            Damage::Mystic => write!(f, "Mystic"),
            Damage::Penetration => write!(f, "Penetration"),
            Damage::Unknown(string) => write!(f, "{string}"),
        }
    }
}

/**
    **This is a `enum` that contains the current Blue Archive armor represented in the API.**

    As of the `14th of December, 2022`,
    this is the current list of armor represented in the API.
    * **Explosion**
    * **Mystic**
    * **Penetration**

    In the case that a armor in the API is not present on the wrapper,
    a [`Armor::Unknown(String)`] is returned to represent the unknown armor with its name in the `enum`.
*/
pub enum Armor {
    Heavy,
    Light,
    Special,
    Unknown(String),
}

impl std::fmt::Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Armor::Heavy => write!(f, "Heavy"),
            Armor::Light => write!(f, "Light"),
            Armor::Special => write!(f, "Special"),
            Armor::Unknown(string) => write!(f, "{string}"),
        }
    }
}

/**
    **This is a `enum` that contains the current Blue Archive clubs represented in the API.**
    As of the `3rd of March, 2023`,
    this is the current list of armor represented in the API.

    * **Kohshinjo68**
    * **SPTF**
    * **GourmetClub**
    * **RemedialClass**
    * **Fuuki**
    * **Countermeasure**
    * **Veritas**
    * **CleanNClearing**
    * **Meihuayuan**
    * **TrainingClub**
    * **Justice**
    * **NinpoKenkyubu**
    * **GameDev**
    * **RedwinterSecretary**
    * **HoukagoDessert**
    * **EmptyClub**
    * **Shugyobu**
    * **BookClub**
    * **SisterHood**
    * **RabbitPlatoon**
    * **AriusSqud**
    * **Onmyobu**
    * **TheSeminar**
    * **Anzenkyoku**
    * **Engineer**
    * **TrinityVigilance**
    * **MatsuriOffice**
    * **Endanbou**
    * **Class227**
    * **Emergentology**
    * **PandemoniumSociety**
    * **FoodService**
    * **KnightsHospitalle**

    In the case that a armor in the API is not present on the wrapper,
    a [`Club::Unknown(String)`] is returned to represent the unknown armor with its name in the `enum`.
*/
#[derive(Debug, Eq, Hash, PartialEq, EnumIter, EnumString, Display)]
pub enum Club {
    #[strum(serialize = "Kohshinjo68")]
    ProblemSolver68,
    #[strum(serialize = "SPTF")]
    SuperPhenomenonTaskForce,
    #[strum(serialize = "GourmetClub")]
    GourmetResearchSociety,
    RemedialClass,
    Fuuki,
    Countermeasure,
    Veritas,
    #[strum(serialize = "CleanNClearing")]
    CleaningAndClearing,
    Meihuayuan,
    TrainingClub,
    Justice,
    NinpoKenkyubu,
    GameDev,
    RedwinterSecretary,
    HoukagoDessert,
    EmptyClub,
    Shugyobu,
    BookClub,
    SisterHood,
    RabbitPlatoon,
    AriusSqud,
    Onmyobu,
    TheSeminar,
    #[strum(serialize = "anzenkyoku")]
    Anzenkyoku,
    Engineer,
    TrinityVigilance,
    MatsuriOffice,
    Endanbou,
    Class227,
    Emergentology,
    PandemoniumSociety,
    FoodService,
    KnightsHospitaller,
    Unknown(String),
}

impl std::fmt::Display for Club {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Club::GourmetClub => write!(f, ""),
            Club::RemedialClass => write!(f, "Make-Up Work Club"),
            Club::Fuuki => write!(f, "School Lunch Club"), // I'm unsure if this is the right one.
            Club::Countermeasure => write!(f, "Foreclosure Task Force"),
            Club::Veritas => write!(f, "Veritas"),
            Club::CleanNClearing => write!(f, "Cleaning & Clearing"),
            Club::Meihuayuan => write!(f, "Plum Blossom Garden"),
            Club::TrainingClub => write!(f, "Athletics Training Club"),
            Club::Justice => write!(f, "Justice Realization Committee"),
            Club::NinpoKenkyubu => write!(f, "Ninjutsu Research Club"),
            Club::GameDev => write!(f, "Game Development Department"),
            Club::RedwinterSecretary => write!(f, "Red Winter Office"), // <- Not sure, we may have to check for the students in each club. // HashMap Club -> Vec<Student> to find out.
            Club::HoukagoDessert => write!(f, "After-School Sweets Club"),
            Club::EmptyClub => write!(f, "Unassigned"),
            Club::Shugyobu => write!(f, "Inner Discipline Club"),
            Club::BookClub => write!(f, "Library Committee"),
            Club::SisterHood => write!(f, "Sisterhood"),
            Club::RabbitPlatoon => write!(f, "RABBIT Squad"),
            Club::AriusSqud => write!(f, "Arius Squad"),
            Club::Onmyobu => write!(f, ""),
            Club::TheSeminar => write!(f, ""),
            Club::Anzenkyoku => write!(f, ""),
            Club::Engineer => write!(f, ""),
            Club::TrinityVigilance => write!(f, ""),
            Club::MatsuriOffice => write!(f, ""),
            Club::Endanbou => write!(f, ""),
            Club::Class227 => write!(f, ""),
            Club::Emergentology => write!(f, ""),
            Club::PandemoniumSociety => write!(f, ""),
            Club::FoodService => write!(f, ""),
            Club::KnightsHospitaller => write!(f, "Knight Hospitaller"),
            Club::Unknown(_) => write!(f, ""),
        }
    }
}
