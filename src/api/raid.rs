use std::borrow::Borrow;

use anyhow::Result;
use reqwest::Client;

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Language, RaidData,
};

/// todo: TBD.
pub async fn fetch_raid_data(
    language: impl Borrow<Language>,
) -> Result<RaidData, BlueArchiveError> {
    Ok(
        fetch_response(&Endpoint::Raids, language.borrow(), &Client::new())
            .await?
            .json::<RaidData>()
            .await?,
    )
}
