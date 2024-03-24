use std::borrow::Borrow;

use crate::types::equipment::{Equipment, EquipmentCategory};

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, Language, Result,
};

/** Fetches all equipment in the database. */
pub async fn fetch_all_equipment(
    language: impl Borrow<Language>,
) -> Result<Vec<Equipment>, BlueArchiveError> {
    Ok(
        fetch_response(&Endpoint::Equipment, language.borrow(), &Client::new())
            .await?
            .json::<Vec<Equipment>>()
            .await?,
    )
}

/** Fetches all equipment that is equal to the given **`name`**. */
pub async fn fetch_equipment_by_name(
    language: impl Borrow<Language>,
    name: impl AsRef<str>,
) -> Result<Option<Equipment>, BlueArchiveError> {
    Ok(fetch_all_equipment(language)
        .await?
        .into_iter()
        .find(|equipment| equipment.name.to_lowercase() == name.as_ref().to_lowercase()))
}

/** Fetches all equipment that is equal to the given **[`EquipmentCategory`]**. */
pub async fn fetch_equipment_by_category(
    language: impl Borrow<Language>,
    category: EquipmentCategory,
) -> Result<Vec<Equipment>, BlueArchiveError> {
    Ok(fetch_all_equipment(language)
        .await?
        .into_iter()
        .filter(|equipment| equipment.category == category)
        .collect::<Vec<_>>())
}
