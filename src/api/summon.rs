use crate::types::Summon;

use super::{internal::Endpoint, *};

/// Fetches all **[`Summon`]'s** from the data.
pub async fn fetch_all_summons(language: &Language) -> Result<Vec<Summon>> {
    let response = super::internal::fetch_response(&Endpoint::Summons, language, &Client::new())
        .await?
        .error_for_status()?;
    Ok(response.json::<Vec<Summon>>().await?)
}
