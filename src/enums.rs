//! Enums for external use via querying, and general accessibility.

use serde::{Deserialize, Serialize};
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
#[derive(EnumString, EnumIter, PartialEq, Eq)]
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
    **This is a `enum` that contains the current Blue Archive squads represented in the API.**

    As of the `29th of March, 2023`,
    this is the current list of squads represented in the API.
    * **Special**
    * **Striker**

    In the case that a squad in the API is not present on the wrapper,
    a [`SquadType::Unknown(String)`] is returned to represent the unknown type with its name in the `enum`.
*/
#[derive(EnumString, EnumIter, PartialEq, Eq)]
pub enum SquadType {
    Special,
    Striker,
    Unknown(String),
}

impl std::fmt::Display for SquadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SquadType::Special => write!(f, "Special"),
            SquadType::Striker => write!(f, "Striker"),
            SquadType::Unknown(unknown_type) => write!(f, "{}", unknown_type),
        }
    }
}
/**
    **This is a `enum` that contains the current Blue Archive schools represented in the API.**

    As of the `4th of April, 2023`,
    this is the current list of schools represented in the API.
    * **Abydos** High School
    * **Gehenna** Academy
    * **Hyakkiyako** Alliance Academy
    * **Millennium** Science School
    * **Shanhaijing** Senior Secondary School
    * **Trinity** General School
    * **Red Winter** Federal Academy
    * **Valkyrie** Police Academy
    * **Arius** Branch School
    * **SRT** Special Academy

    In the case that a school in the API is not present on the wrapper,
    a [`School::Unknown(String)`] is returned to represent the unknown school with its name in the `enum`.
*/
#[derive(EnumString, Debug, Display, EnumIter, PartialEq, Eq)]
pub enum School {
    /// **Abydos** High School
    Abydos,
    /// **Gehenna** Academy
    Gehenna,
    /// **Hyakkiyako** Alliance Academy
    Hyakkiyako,
    /// **Millennium** Science School
    Millennium,
    /// **Shanhaijing** Senior Secondary School
    Shanhaijing,
    /// **Trinity** General School
    Trinity,
    /// **Red Winter** Federal Academy
    RedWinter,
    /// **Valkyrie** Police Academy
    Valkyrie,
    /// **Arius** Branch School
    Arius,
    /// **SRT** Special Academy
    SRT,
    Unknown(String),
}

impl School {
    /// The full name of the school.
    pub fn full_name(&self) -> String {
        let name = match self {
            School::Abydos => "Abydos High School",
            School::Gehenna => "Gehenna Academy",
            School::Hyakkiyako => "Hyakkiyako Alliance Academy",
            School::Millennium => "Millennium Science School",
            School::Shanhaijing => "Shanhaijing Senior Secondary School",
            School::Trinity => "Trinity General School",
            School::RedWinter => "Red Winter Federal Academy",
            School::Valkyrie => "Valkyrie Police Academy",
            School::Arius => "Arius Branch School",
            School::SRT => "SRT Special Academy",
            School::Unknown(string) => string.as_ref(),
        };
        name.to_string()
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
#[derive(EnumString, EnumIter, PartialEq, Eq)]
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
#[derive(EnumString, Display, EnumIter, PartialEq, Eq)]
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
    /// The full name of the weapon.
    pub fn full_name(&self) -> String {
        let name = match self {
            Weapon::AR => "Assault Rifle",
            Weapon::GL => "Grenade Launcher",
            Weapon::HG => "Handgun",
            Weapon::MG => "Machine Gun",
            Weapon::MT => "Mortar",
            Weapon::RF => "Rifle",
            Weapon::RG => "Railgun",
            Weapon::RL => "Rocket Launcher",
            Weapon::SG => "Shotgun",
            Weapon::SMG => "Submachine Gun",
            Weapon::SR => "Sniper Rifle",
            Weapon::Unknown(string) => string,
        };
        name.to_string()
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
#[derive(EnumString, EnumIter, PartialEq, Eq)]
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

    As of the `3rd of April, 2023`,
    this is the current list of armor represented in the API.
    * **Explosion**
    * **Mystic**
    * **Penetration**

    In the case that a armor in the API is not present on the wrapper,
    a [`Armor::Unknown(String)`] is returned to represent the unknown armor with its name in the `enum`.
*/

#[derive(EnumString, EnumIter, PartialEq, Eq)]
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

    As of the `4th of April, 2023`,
    this is the current list of armor represented in the API.

    - **Problem Solver 68**
    - **Super Phenomenon Task Force**
    - **Gourmet Research Society**
    - **Make Up Work Club**
    - **Prefect Team**
    - **Foreclosure Task Force**
    - **Veritas**
    - **Cleaning & Clearing**
    - **Plum Blossom Garden**
    - **Athletics Training Club**
    - **Ninjutsu Research Department**
    - **Justice Task Force**
    - **Game Development Department**
    - **Red Winter Secretary**
    - **After School Sweets Club**
    - **Unassigned**
    - **Inner Discipline Club**
    - **Library Committee**
    - **Sisterhood**
    - **RABBIT Squad**
    - **Arius Squad**
    - **Yin-Yang Club**
    - **Seminar**
    - **Public Safety Bureau**
    - **Engineering Club**
    - **Trinity Vigilante Crew**
    - **Festival Operations Department**
    - **Chinese Alchemy Study Club**
    - **Spec Ops #227**
    - **Medical Emergency Club**
    - **Pandemonium Society**
    - **School Lunch Club**
    - **Remedial Knights**

    In the case that a club in the API is not present on the wrapper,
    a [`Club::Unknown(String)`] is returned to represent the unknown club with its name in the `enum`.
*/
#[derive(Debug, Hash, Eq, PartialEq, EnumIter, EnumString, Display)]
pub enum Club {
    #[strum(serialize = "Kohshinjo68", to_string = "Problem Solver 68")]
    ProblemSolver68,
    #[strum(serialize = "SPTF", to_string = "Super Phenomenon Task Force")]
    SuperPhenomenonTaskForce,
    #[strum(serialize = "GourmetClub", to_string = "Gourmet Research Society")]
    GourmetResearchSociety,
    #[strum(serialize = "RemedialClass", to_string = "Make-Up Work Club")]
    MakeUpWorkClub,
    #[strum(serialize = "Fuuki", to_string = "Prefect Team")]
    PrefectTeam,
    #[strum(serialize = "Countermeasure", to_string = "Foreclosure Task Force")]
    ForeclosureTaskForce,
    Veritas,
    #[strum(serialize = "CleanNClearing", to_string = "Cleaning & Clearing")]
    CleaningAndClearing,
    #[strum(serialize = "Meihuayuan", to_string = "Plum Blossom Garden")]
    PlumBlossomGarden,
    #[strum(serialize = "TrainingClub", to_string = "Athletics Training Club")]
    AthleticsTrainingClub,
    #[strum(serialize = "NinpoKenkyubu", to_string = "Ninjutsu Research Club")]
    NinjutsuResearchDepartment,
    #[strum(serialize = "Justice", to_string = "Justice Task Force")]
    JusticeTaskForce,
    #[strum(serialize = "GameDev", to_string = "Game Development Department")]
    GameDevelopmentDepartment,
    #[strum(serialize = "RedwinterSecretary", to_string = "Red Winter Office")]
    RedWinterSecretary,
    #[strum(serialize = "HoukagoDessert", to_string = "After-School Sweets Club")]
    AfterSchoolSweetsClub,
    #[strum(serialize = "EmptyClub", to_string = "Unassigned")]
    Unassigned,
    #[strum(serialize = "Shugyobu", to_string = "Inner Discipline Club")]
    InnerDisciplineClub,
    #[strum(serialize = "BookClub", to_string = "Library Committee")]
    LibraryCommittee,
    #[strum(serialize = "SisterHood", to_string = "Sisterhood")]
    Sisterhood,
    #[strum(serialize = "RabbitPlatoon", to_string = "RABBIT Squad")]
    RABBITSquad,
    #[strum(serialize = "AriusSqud", to_string = "Arius Squad")]
    AriusSquad,
    #[strum(serialize = "Onmyobu", to_string = "Yin-Yang Club")]
    YinYangClub,
    #[strum(serialize = "TheSeminar", to_string = "Seminar")]
    Seminar,
    #[strum(serialize = "anzenkyoku", to_string = "Public Safety Bureau")]
    PublicSafetyBureau,
    #[strum(serialize = "Engineer", to_string = "Engineering Club")]
    EngineeringClub,
    #[strum(serialize = "TrinityVigilance", to_string = "Trinity Vigilante Crew")]
    TrinityVigilanteCrew,
    #[strum(
        serialize = "MatsuriOffice",
        to_string = "Festival Operations Department"
    )]
    FestivalOperationsDepartment,
    #[strum(serialize = "Endanbou", to_string = "Chinese Alchemy Study Club")]
    ChineseAlchemyStudyClub,
    #[strum(serialize = "Class227", to_string = "Spec Ops #227")]
    SpecOpsNumber227,
    #[strum(serialize = "Emergentology", to_string = "Medical Emergency Club")]
    MedicalEmergencyClub,
    #[strum(serialize = "FoodService", to_string = "School Lunch Club")]
    SchoolLunchClub,
    #[strum(serialize = "PandemoniumSociety", to_string = "Pandemonium Society")]
    PandemoniumSociety,
    #[strum(serialize = "KnightsHospitaller", to_string = "Remedial Knights")]
    RemedialKnights,
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive rarities represented in the API.**

    As of the `3rd of April, 2023`,
    this is the current list of armor represented in the API.

    * **R** (Rare)
    * **SR** (Super Rare)
    * **SSR** (Super-Super Rare)

    In the case that a rarity in the API is not present on the wrapper,
    a [`Rarity::Unknown(String)`] is returned to represent the unknown rarity with its name in the `enum`.
*/
#[derive(Clone, Debug, Display, Deserialize, Serialize, EnumString, Eq, PartialEq)]
pub enum Rarity {
    R,
    SR,
    SSR,
    Unknown(String),
}

impl Rarity {
    /// The full name of the rarity type.
    pub fn full_name(&self) -> String {
        let name = match self {
            Rarity::R => "Rare",
            Rarity::SR => "Super Rare",
            Rarity::SSR => "Super-Super Rare",
            Rarity::Unknown(string) => string,
        };
        name.to_string()
    }
}
