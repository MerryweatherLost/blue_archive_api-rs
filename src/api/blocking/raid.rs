//! Functions primarily for geting [`RaidData`].

use std::borrow::Borrow;

use anyhow::Result;

use crate::{types::RaidData, Language};

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client,
};

/// Fetches **[`RaidData`]**, which contains information related to raids in Blue Archive.
pub fn get_raid_data(language: impl Borrow<Language>) -> Result<RaidData, BlueArchiveError> {
    Ok(get_response(&Endpoint::Raids, language.borrow(), &Client::new())?.json::<RaidData>()?)
}
