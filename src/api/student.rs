//! Functions primarily for fetching [`Student`] data.

use std::borrow::Borrow;

use super::{
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, IteratorRandom, Language, Result, Student, StudentFilterOptions,
};

/// Fetches all students with extra data, which includes the images of the **[`Students`][`Student`]** among other things.
pub async fn fetch_all_students(
    language: impl Borrow<Language>,
) -> Result<Vec<Student>, BlueArchiveError> {
    let client = Client::new();
    let mut students = fetch_response(&Endpoint::Students, language.borrow(), &client)
        .await?
        .json::<Vec<Student>>()
        .await?;
    futures::future::join_all(
        students
            .iter_mut()
            .map(|student| async { student.fetch_extra_data(&client).await }),
    )
    .await;
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

    #[tokio::main]
    async fn main() -> Result<()> {
        // Fetching asuna is relatively simple...
        let asuna = blue_archive::fetch_student_by_name("Asuna", Language::English).await?.unwrap();
        println!("{}", asuna);
        Ok(())
    }
    ```
*/
pub async fn fetch_student_by_name(
    name: impl Into<String>,
    language: impl Borrow<Language>,
) -> Result<Option<Student>, BlueArchiveError> {
    let name: String = name.into();
    let possible_student = fetch_all_students(language)
        .await?
        .into_iter()
        .find(|student| {
            [
                &student.name,
                &student.first_name(),
                &student.last_name(),
                &student.full_name_last(),
                &student.full_name_first(),
            ]
            .into_iter()
            .any(|x| x.to_lowercase() == name.to_lowercase())
        });
    match possible_student {
        Some(mut student) => {
            student.fetch_extra_data(&Client::new()).await?;
            Ok(Some(student))
        }
        None => Ok(None),
    }
}

/// Attempts to fetch a random **[`Student`]**.
///
/// If fetching the data fails, then it will return a [`BlueArchiveError`], as the data would not have any **[`Students`][`Student`]**.
pub async fn fetch_random_student(
    language: impl Borrow<Language>,
) -> Result<Option<Student>, BlueArchiveError> {
    Ok(fetch_all_students(language)
        .await?
        .into_iter()
        .choose(&mut rand::thread_rng()))
}

/// Attempts to fetch a random amount of **[`Students`][`Student`]** depending on the specified **`amount`**.
///
/// Depending on the **`amount`** inserted, if it exceeds the total length of **[`Students`][`Student`]** from the data, it will just return everything.
///
/// If fetching the data fails, then it will return a [`BlueArchiveError`], as the data would not have any **[`Students`][`Student`]**.
pub async fn fetch_random_students(
    language: impl Borrow<Language>,
    amount: usize,
) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(fetch_all_students(language)
        .await?
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), amount))
}

/// Returns **[`StudentFilterOptions`]** to be used with the provided **[`Vec<Student>`]** for filtering.
pub fn filter(students: &Vec<Student>) -> StudentFilterOptions {
    StudentFilterOptions::new(students)
}
