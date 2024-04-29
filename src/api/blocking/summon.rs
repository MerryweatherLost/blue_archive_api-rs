//! Functions primarily for geting [`Summon`] data.

use std::borrow::Borrow;

use crate::{types::Summon, Language};

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client,
};

/// Fetches all **[`Summons`][`Summon`]** from the data.
pub fn get_all_summons(language: impl Borrow<Language>) -> Result<Vec<Summon>, BlueArchiveError> {
    Ok(
        get_response(&Endpoint::Summons, language.borrow(), &Client::new())?
            .json::<Vec<Summon>>()?,
    )
}
