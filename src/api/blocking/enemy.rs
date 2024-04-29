use std::borrow::Borrow;

use crate::types::enemy::Enemy;

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client, Result,
};

use crate::Language;

/// Fetches all [`Enemy`]'s that are currently in the database.
pub fn get_all_enemies(language: impl Borrow<Language>) -> Result<Vec<Enemy>, BlueArchiveError> {
    Ok(
        get_response(&Endpoint::Enemies, language.borrow(), &Client::new())?
            .json::<Vec<Enemy>>()?,
    )
}

/**
   Fetches a specific **[`Enemy`]** that matches with a provided **`name`** argument.


*/
pub fn get_enemy_by_name(
    language: impl Borrow<Language>,
    name: impl AsRef<str>,
) -> Result<Option<Enemy>, BlueArchiveError> {
    Ok(get_all_enemies(language)?
        .into_iter()
        .find(|enemy| enemy.name.to_lowercase() == name.as_ref().to_lowercase()))
}
