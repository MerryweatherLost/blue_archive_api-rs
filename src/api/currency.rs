use std::borrow::Borrow;

use crate::types::currency::Currency;

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, Language, Result,
};

/**
    Fetches all existing **[`Currency`]** currently in the database.

    # Examples
    ```
        use blue_archive::Language;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            println!(
                "Total Currencies: [{}]",
                blue_archive::fetch_all_currencies(Language::English)
                    .await?
                    .len()
            );
            Ok(())
        }
    ```
*/
pub async fn fetch_all_currencies(
    language: impl Borrow<Language>,
) -> Result<Vec<Currency>, BlueArchiveError> {
    Ok(
        fetch_response(&Endpoint::Currency, language.borrow(), &Client::new())
            .await?
            .json::<Vec<Currency>>()
            .await?,
    )
}

/**
    Fetches a specific **[`Currency`]** that matches with a provided **`name`** argument.

    # Examples
    ```
        use blue_archive::Language;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            let pyroxenes_now = blue_archive::fetch_currency_by_name("Pyroxenes", Language::English)
                .await?
                .unwrap();
            println!("Pyroxenes");
            println!("--------------------------");
            println!("{:?}", pyroxenes_now);
            Ok(())
        }
    ```
*/
pub async fn fetch_currency_by_name(
    name: impl Into<String>,
    language: impl Borrow<Language>,
) -> Result<Option<Currency>, BlueArchiveError> {
    let name: String = name.into();
    Ok(fetch_all_currencies(language)
        .await?
        .into_iter()
        .find(|currency| currency.name.to_lowercase() == name.to_lowercase()))
}
