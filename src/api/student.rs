//! Functions primarily for fetching [`Student`] data.

use std::borrow::Borrow;

use super::{
    internal,
    internal::{fetch_response, Endpoint},
    BlueArchiveError, Client, IteratorRandom, Language, Result, Student, StudentFilterOptions,
};

/// **Obtains all Students without adding extra content**.
///
/// Extra content such as images will not be retrieved unless the **[`Students`][`Student`]** are mutated and are fetched by using `fetch_extra_data`.
/// It can be beneficial to use this in a context where you want information retrieved as quickly as possible.
pub async fn fetch_all_students_without_extra(
    language: impl Borrow<Language>,
) -> Result<Vec<Student>, BlueArchiveError> {
    let response = internal::fetch_response(&Endpoint::Students, language.borrow(), &Client::new())
        .await?
        .error_for_status()?;
    Ok(response.json::<Vec<Student>>().await?)
}

/// Fetches all students with extra data, which includes the images of the **[`Students`][`Student`]** among other things.
pub async fn fetch_all_students(
    language: impl std::borrow::Borrow<Language>,
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

    It is recommended to use the last name and an associated tag such as **`Iori (Swimsuit)`** of the **[`Student`]**.

    /// ## Different Methods
    /// - Searching with an associated tag, such as **`Iori (Swimsuit)`**
    ///     - It is recommended to use the last name and an associated tag if you are looking for a **[`Student`]** with a different appearance.
    /// - Searching via. the **last name (`surname`)**.
    /// - Searching via. the **first name**.
    /// - Searching via. the **first name and last name together**, and vise versa (e.g. Ichinose Asuna/Asuna Ichinose).
    ///
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
    let possible_student = fetch_all_students_without_extra(language)
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

/// Returns **[`StudentFilterOptions`]** to be used with the provided **[`Vec<Student>`]** for filtering.
pub fn filter(students: &Vec<Student>) -> StudentFilterOptions {
    StudentFilterOptions::new(students)
}
