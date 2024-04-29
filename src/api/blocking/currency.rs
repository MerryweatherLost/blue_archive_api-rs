use std::borrow::Borrow;

use crate::types::currency::Currency;

use crate::Language;

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client, Result,
};

/**
    Fetches all existing **[`Currency`]** currently in the database.

    # Examples
    ```
        use blue_archive::Language;

        fn main() -> anyhow::Result<()> {
            println!(
                "Total Currencies: [{}]",
                blue_archive::blocking::get_all_currencies(Language::English).len()
            );
            Ok(())
        }
    ```
*/
fn get_all_currencies(language: impl Borrow<Language>) -> Result<Vec<Currency>, BlueArchiveError> {
    Ok(
        get_response(&Endpoint::Currency, language.borrow(), &Client::new())?
            .json::<Vec<Currency>>()?,
    )
}

/**
    Fetches a specific **[`Currency`]** that matches with a provided **`name`** argument.

    # Examples
    ```
        use blue_archive::Language;

        fn main() -> anyhow::Result<()> {
            let pyroxenes_now = blue_archive::blocking::get_currency_by_name("Pyroxenes", Language::English)
                .unwrap();
            println!("Pyroxenes");
            println!("--------------------------");
            println!("{:?}", pyroxenes_now);
            Ok(())
        }
    ```
*/
pub fn get_currency_by_name(
    name: impl AsRef<str>,
    language: impl Borrow<Language>,
) -> Result<Option<Currency>, BlueArchiveError> {
    Ok(get_all_currencies(language)?
        .into_iter()
        .find(|currency| currency.name.to_lowercase() == name.as_ref().to_lowercase()))
}
