use std::borrow::Borrow;

use crate::types::enemy::Enemy;

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, Language, Result,
};

/// Fetches all [`Enemy`]'s that are currently in the database.
pub async fn fetch_all_enemies(
    language: impl Borrow<Language>,
) -> Result<Vec<Enemy>, BlueArchiveError> {
    Ok(
        fetch_response(&Endpoint::Enemies, language.borrow(), &Client::new())
            .await?
            .json::<Vec<Enemy>>()
            .await?,
    )
}

/**
   Fetches a specific **[`Enemy`]** that matches with a provided **`name`** argument.


*/
pub async fn fetch_enemy_by_name(
    language: impl Borrow<Language>,
    name: impl AsRef<str>,
) -> Result<Option<Enemy>, BlueArchiveError> {
    Ok(fetch_all_enemies(language)
        .await?
        .into_iter()
        .find(|enemy| enemy.name.to_lowercase() == name.as_ref().to_lowercase()))
}
