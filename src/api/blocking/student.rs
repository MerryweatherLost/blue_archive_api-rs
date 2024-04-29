//! Functions primarily for geting [`Student`] data.

use std::borrow::Borrow;

use rand::seq::IteratorRandom;

use crate::{
    filter::student::StudentFilterOptions,
    types::{students::student::StudentImageData, Student},
    Language,
};

use super::{
    internal::{get_response, Endpoint},
    BlueArchiveError, Client, Result,
};

/// Fetches all students with extra data, which includes the images of the **[`Students`][`Student`]** among other things.
pub fn get_all_students(language: impl Borrow<Language>) -> Result<Vec<Student>, BlueArchiveError> {
    let client = Client::new();
    let mut students =
        get_response(&Endpoint::Students, language.borrow(), &client)?.json::<Vec<Student>>()?;

    students
        .iter_mut()
        .for_each(|student| student.image = StudentImageData::new(student));

    Ok(students)
}

/**
    Fetches a **[`Student`]** by a `name` from a set of names.

    ## Different Methods
    - Searching with an associated tag, such as **`Iori (Swimsuit)`**
        - It is recommended to use the last name and an associated tag if you are looking for a **[`Student`]** with a different appearance.
    - Searching via. the **last name (`surname`)**.
    - Searching via. the **first name**.
    - Searching via. the **first name and last name together**, and vise versa (e.g. Asuna Ichinose/Ichinose Asuna).

    # Examples
    ```
    use anyhow::Result;

    use blue_archive::Language;

    fn main() -> Result<()> {
        // Fetching asuna is relatively simple...
        let asuna = blue_archive::blocking::get_student_by_name("Asuna", Language::English)?.unwrap();
        println!("{}", asuna);
        Ok(())
    }
    ```
*/
pub fn get_student_by_name(
    name: impl AsRef<str>,
    language: impl Borrow<Language>,
) -> Result<Option<Student>, BlueArchiveError> {
    let mut matched_student = None;

    for student in get_all_students(language)? {
        let lowercased = name.as_ref().to_lowercase();
        let maybe_student = (lowercased == student.name.to_lowercase()
            || lowercased == student.first_name.to_lowercase()
            || lowercased == student.last_name.to_lowercase()
            || lowercased == student.full_name_last().to_lowercase()
            || lowercased == student.full_name_first().to_lowercase())
        .then_some(student);
        if let Some(student) = maybe_student {
            matched_student = Some(student);
            break;
        }
    }

    Ok(matched_student)
}

/// Attempts to get a random **[`Student`]**.
///
/// If geting the data fails, then it will return a [`BlueArchiveError`], as the data would not have any **[`Students`][`Student`]**.
pub fn get_random_student(
    language: impl Borrow<Language>,
) -> Result<Option<Student>, BlueArchiveError> {
    Ok(get_all_students(language)?
        .into_iter()
        .choose(&mut rand::thread_rng()))
}

/// Attempts to get a random amount of **[`Students`][`Student`]** depending on the specified **`amount`**.
///
/// Depending on the **`amount`** inserted, if it exceeds the total length of **[`Students`][`Student`]** from the data, it will just return everything.
///
/// If geting the data fails, then it will return a [`BlueArchiveError`], as the data would not have any **[`Students`][`Student`]**.
pub fn get_random_students(
    language: impl Borrow<Language>,
    amount: usize,
) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(get_all_students(language)?
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), amount))
}

/// Returns **[`StudentFilterOptions`]** to be used with the provided **[`Vec<Student>`]** for filtering.
pub fn filter(students: &Vec<Student>) -> StudentFilterOptions {
    StudentFilterOptions::new(students)
}
