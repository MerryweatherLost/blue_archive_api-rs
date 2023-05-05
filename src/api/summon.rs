//! Functions primarily for fetching [`Summon`] data.

use crate::types::Summon;

use super::{internal::Endpoint, Client, Language, Result};

/// Fetches all **[`Summons`][`Summon`]** from the data.
pub async fn fetch_all_summons(language: &Language) -> Result<Vec<Summon>> {
    let response =
        super::internal::fetch_response(&Endpoint::Summons, language, &Client::new()).await?;
    Ok(response.json::<Vec<Summon>>().await?)
}
