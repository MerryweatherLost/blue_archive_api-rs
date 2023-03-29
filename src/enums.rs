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
#[derive(EnumString, EnumIter)]
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
#[derive(EnumString, EnumIter)]
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
#[derive(EnumString, EnumIter)]
pub enum School {
    Abydos,
    Gehenna,
    Hyakkiyako,
    Millennium,
    Shanhaijing,
    Trinity,
    Unknown(String),
}

impl School {
    pub fn full_name(&self) -> String {
        let name = match self {
            School::Abydos => "Abydos High School",
            School::Gehenna => "Gehenna Academy",
            School::Hyakkiyako => "Hyakkiyako Alliance Academy",
            School::Millennium => "Millennium Science School",
            School::Shanhaijing => "Shanhaijing Senior Secondary School",
            School::Trinity => "Trinity General School",
            School::Unknown(string) => string.as_ref(),
        };
        name.to_string()
    }
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
#[derive(EnumString, EnumIter)]
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
#[derive(EnumString, Display, EnumIter)]
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
            Weapon::Unknown(string) => string.as_str(),
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
#[derive(EnumString, EnumIter)]
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

#[derive(EnumString, EnumIter)]
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
    Kohshinjo68,
    SPTF,
    GourmetClub,
    RemedialClass,
    Fuuki,
    Countermeasure,
    Veritas,
    CleaningNClearing,
    Meihuayuan,
    TrainingClub,
    Justice,
    NinpoKenkyubu,
    GameDev,
    RedwinterSecretary,
    HoukagoDessert,
    #[strum(serialize = "EmptyClub")]
    Unassigned,
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
