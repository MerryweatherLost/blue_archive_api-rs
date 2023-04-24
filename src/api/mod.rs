pub use reqwest::{Request, StatusCode};

pub(crate) mod internal {
    /// Contains the endpoints for the data, they mainly just represent the path of what data is obtained.
    enum Endpoints {
        Currency,
        Enemies,
        Equipment,
        Furniture,
        Items,
        Localization,
        Raids,
        Students,
        Summons,
    }

    impl std::fmt::Display for Endpoints {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Endpoints::Currency => todo!(),
                Endpoints::Enemies => todo!(),
                Endpoints::Equipment => todo!(),
                Endpoints::Furniture => todo!(),
                Endpoints::Items => todo!(),
                Endpoints::Localization => todo!(),
                Endpoints::Raids => todo!(),
                Endpoints::Students => todo!(),
                Endpoints::Summons => todo!(),
            }
        }
    }

    fn fetch_response(endpoint: &Endpoints) {}
}
