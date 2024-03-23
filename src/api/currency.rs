use std::borrow::Borrow;

use crate::types::currency::Currency;

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, Language, Result,
};

/**
    Fetches all existing **[`Currency`]** currently in the database.
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
