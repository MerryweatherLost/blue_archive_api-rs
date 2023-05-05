//! Contains useful public enums to be used when working with the API wrapper.

use strum_macros::{Display, EnumIter, EnumString};

/// Languages that **``SchaleDB``** supports.
#[derive(Debug, Display)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
    Thai,
    Taiwanese,
    Vietnamese,
}

impl Language {
    /// The identifier of the Language, e.g. (en, tw, kr)
    pub fn id(&self) -> String {
        match self {
            Self::English => "en",
            Self::Chinese => "cn",
            Self::Japanese => "jp",
            Self::Korean => "kr",
            Self::Thai => "th",
            Self::Taiwanese => "tw",
            Self::Vietnamese => "vi",
        }
        .to_string()
    }
}

/**
    **This is a `enum` that contains the current Blue Archive schools represented in the data.**

    This is the current list of schools represented in the data.
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

    In the case that a school in the data is not present on the wrapper,
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
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

impl School {
    /// The full name of the school.
    pub fn full_name(&self) -> String {
        match self {
            Self::Abydos => "Abydos High School",
            Self::Gehenna => "Gehenna Academy",
            Self::Hyakkiyako => "Hyakkiyako Alliance Academy",
            Self::Millennium => "Millennium Science School",
            Self::Shanhaijing => "Shanhaijing Senior Secondary School",
            Self::Trinity => "Trinity General School",
            Self::RedWinter => "Red Winter Federal Academy",
            Self::Valkyrie => "Valkyrie Police Academy",
            Self::Arius => "Arius Branch School",
            Self::SRT => "SRT Special Academy",
            Self::Unknown(string) => string.as_ref(),
        }
        .to_string()
    }
}

/**
    **This is a `enum` that contains the current Blue Archive tactical roles represented in the data.**

    This is the list of current tactical roles represented in the data.
    - **Tanker**
    - **Vehicle**
    - **DamageDealer**
    - **Healer**
    - **Supporter**

    In the case that a tactical role in the data is not present on the wrapper,
    a [`TacticalRole::Unknown(String)`] is returned to represent the unknown tactical role with its name in the `enum`.
*/
#[derive(Debug, Display, EnumString, EnumIter, PartialEq, Eq)]
pub enum TacticalRole {
    Tanker,
    Vehicle,
    #[strum(serialize = "DamageDealer", to_string = "Damage Dealer")]
    DamageDealer,
    Healer,
    Supporter,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive squads represented in the data.**

    This is the current list of squads represented in the data.
    * **Main** (Striker)
    * **Support** (Special)

    In the case that a squad in the data is not present on the wrapper,
    a [`Squad::Unknown(String)`] is returned to represent the unknown type with its name in the `enum`.
*/
#[derive(Debug, Display, EnumString, EnumIter, PartialEq, Eq)]
pub enum Squad {
    Main,
    Support,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

impl Squad {
    /// Obtains an alternative name for the [Squad] that some may be familiar with.
    ///
    /// - **Main** -> Striker
    /// - **Support** -> Special
    ///
    /// Although, if [``Squad::Unknown``], the inner value will just be returned.
    pub fn alt_name(&self) -> String {
        match self {
            Self::Main => "Striker".to_string(),
            Self::Support => "Special".to_string(),
            Self::Unknown(squad) => squad.to_string(),
        }
    }
}

/**
    **This is a `enum` that contains the current Blue Archive armor represented in the data.**

    This is the current list of armor represented in the data.
    * **Unarmed**
    * **ElasticArmor**
    * **HeavyArmor**
    * **LightArmor**

    In the case that a armor in the data is not present on the wrapper,
    a [`Armor::Unknown(String)`] is returned to represent the unknown armor with its name in the `enum`.
*/
#[derive(Debug, Display, EnumString, EnumIter, PartialEq, Eq)]
pub enum Armor {
    Unarmed,
    #[strum(serialize = "ElasticArmor", to_string = "Elastic Armor")]
    ElasticArmor,
    #[strum(serialize = "HeavyArmor", to_string = "Heavy Armor")]
    HeavyArmor,
    #[strum(serialize = "LightArmor", to_string = "Light Armor")]
    LightArmor,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive positions represented in the data.**

    This is the current list of positions represented in the data.
    * **Front**
    * **Middle**
    * **Back**

    In the case that a position in the data is not present on the wrapper,
    a [`Position::Unknown(String)`] is returned to represent the unknown position with its name in the `enum`.
*/
#[derive(Debug, Display, EnumString, EnumIter, PartialEq, Eq)]
pub enum Position {
    Front,
    Middle,
    Back,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive bullet types represented in the data.**

    This is the current list of bullet types represented in the data.
    * **Explosion**
    * **Mystic**
    * **Piercing**

    In the case that a bullet type in the data is not present on the wrapper,
    a [`BulletType::Unknown(String)`] is returned to represent the unknown bullet type with its name in the `enum`.
*/
#[derive(Debug, Display, EnumString, EnumIter, PartialEq, Eq)]
pub enum BulletType {
    Explosion,
    Mystic,
    Penetration,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive clubs represented in the data.**

    This is the current list of clubs represented in the data.
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

    In the case that a club in the data is not present on the wrapper,
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
    NinjutsuResearchClub,
    #[strum(serialize = "Justice", to_string = "Justice Task Force")]
    JusticeTaskForce,
    #[strum(serialize = "GameDev", to_string = "Game Development Club")]
    GameDevelopmentClub,
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
    #[strum(serialize = "PublicPeaceBureau", to_string = "Public Peace Bureau")]
    PublicPeaceBureau,
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
    #[strum(serialize = "TeaParty", to_string = "Tea Party")]
    TeaParty,
    #[strum(serialize = "HotSpringsDepartment", to_string = "Hot Spring Club")]
    HotSpringsClub,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

/**
    **This is a `enum` that contains the current Blue Archive weapon types represented in the data.**

    This is the current list of weapon types represented in the data.
    * **AR** (Assault Rifle)
    * **GL** (Grenade Launcher)
    * **FT** (Flamethrower)
    * **HG** (Handgun)
    * **MG** (Machine Gun)
    * **MT** (Mortar)
    * **RG** (Railgun)
    * **RL** (Rocket Launcher)
    * **SG** (Shotgun)
    * **SMG** (Submachine Gun)
    * **SR** (Sniper RIfle)

    In the case that a weapon type in the data is not present on the wrapper,
    a [`WeaponType::Unknown(String)`] is returned to represent the unknown weapon type with its name in the `enum`.
*/
#[derive(Debug, EnumString, Display, EnumIter, PartialEq, Eq)]
pub enum WeaponType {
    /// **`Assault Rifle`**
    AR,
    /// **`Flamethrower`**
    FT,
    /// **`Sniper Rifle`**
    SR,
    /// **`Machine Gun`**
    MG,
    /// **`Mortar`**
    MT,
    /// **`Handgun`**
    HG,
    /// **`Submachine Gun`**
    SMG,
    /// **`Rocket Launcher`**
    RL,
    /// **`Shotgun`**
    SG,
    /// **`Grenade Launcher`**
    GL,
    /// **`Railgun`**
    RG,
    /// An **`unknown`** type that contains the inner value.
    Unknown(String),
}

impl WeaponType {
    /// The full name of the weapon.
    pub fn full_name(&self) -> String {
        match self {
            Self::AR => "Assault Rifle",
            Self::GL => "Grenade Launcher",
            Self::FT => "Flamethrower",
            Self::HG => "Handgun",
            Self::MG => "Machine Gun",
            Self::MT => "Mortar",
            Self::RG => "Railgun",
            Self::RL => "Rocket Launcher",
            Self::SG => "Shotgun",
            Self::SMG => "Submachine Gun",
            Self::SR => "Sniper Rifle",
            Self::Unknown(string) => string,
        }
        .to_string()
    }
}
