use strum_macros::Display;

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
            Language::English => "en",
            Language::Chinese => "cn",
            Language::Japanese => "jp",
            Language::Korean => "kr",
            Language::Thai => "th",
            Language::Taiwanese => "tw",
            Language::Vietnamese => "vi",
        }
        .to_string()
    }
}
