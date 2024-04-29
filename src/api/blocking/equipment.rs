use std::borrow::Borrow;

use crate::types::equipment::{Equipment, EquipmentCategory};

use crate::Language;

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client, Result,
};

/** Fetches all equipment in the database. */
pub fn get_all_equipment(
    language: impl Borrow<Language>,
) -> Result<Vec<Equipment>, BlueArchiveError> {
    Ok(
        get_response(&Endpoint::Equipment, language.borrow(), &Client::new())?
            .json::<Vec<Equipment>>()?,
    )
}

/** Fetches all equipment that is equal to the given **`name`**. */
pub fn get_equipment_by_name(
    language: impl Borrow<Language>,
    name: impl AsRef<str>,
) -> Result<Option<Equipment>, BlueArchiveError> {
    Ok(get_all_equipment(language)?
        .into_iter()
        .find(|equipment| equipment.name.to_lowercase() == name.as_ref().to_lowercase()))
}

/** Fetches all equipment that is equal to the given **[`EquipmentCategory`]**. */
pub fn get_equipment_by_category(
    language: impl Borrow<Language>,
    category: EquipmentCategory,
) -> Result<Vec<Equipment>, BlueArchiveError> {
    Ok(get_all_equipment(language)?
        .into_iter()
        .filter(|equipment| equipment.category == category)
        .collect::<Vec<_>>())
}
