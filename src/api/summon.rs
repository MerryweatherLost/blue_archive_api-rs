//! Functions primarily for fetching [`Summon`] data.

use std::borrow::Borrow;

use super::{internal::Endpoint, BlueArchiveError, Client, Language, Result, Summon};

/// Fetches all **[`Summons`][`Summon`]** from the data.
pub async fn fetch_all_summons(
    language: impl Borrow<Language>,
) -> Result<Vec<Summon>, BlueArchiveError> {
    let response =
        super::internal::fetch_response(&Endpoint::Summons, language.borrow(), &Client::new())
            .await?;
    Ok(response.json::<Vec<Summon>>().await?)
}
