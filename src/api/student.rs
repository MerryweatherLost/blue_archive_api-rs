//! Functions primarily for fetching [`Student`] data.

use super::{
    internal::{fetch_response, Endpoint},
    *,
};

/// **Obtains all Students without adding extra content**.
///
/// Extra content such as images will not be retrieved unless the **[`Students`][`Student`]** are mutated and are fetched by using `fetch_extra_data`.
/// It can be beneficial to use this in a context where you want information retrieved as quickly as possible.
pub async fn fetch_all_students_without_extra(
    language: &Language,
) -> Result<Vec<Student>, BlueArchiveError> {
    let response = internal::fetch_response(&Endpoint::Students, language, &Client::new())
        .await?
        .error_for_status()?;
    Ok(response.json::<Vec<Student>>().await?)
}

/// Lets you fetch all students with extra data, which includes the images of the **[`Students`][`Student`]** among other things.
pub async fn fetch_all_students(language: &Language) -> Result<Vec<Student>, BlueArchiveError> {
    let client = Client::new();
    let mut students = fetch_response(&Endpoint::Students, language, &client)
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

/// Fetches a **[`Student`]** by a `name` from a set of pattern matching.
pub async fn fetch_student_by_name(
    name: impl Into<String>,
    language: &Language,
) -> Result<Option<Student>, BlueArchiveError> {
    let name: String = name.into();
    let possible_student = fetch_all_students_without_extra(language)
        .await?
        .into_iter()
        .find(|student| {
            [
                &student.first_name(),
                &student.last_name(),
                &student.full_name_with_last(),
                &student.full_name_with_first(),
            ]
            .into_iter()
            .any(|x| x.to_lowercase() == name.to_lowercase())
        });
    match possible_student {
        Some(mut student) => {
            student.fetch_extra_data(&Client::new()).await;
            Ok(Some(student))
        }
        None => Ok(None),
    }
}

/// Attempts to fetch a random **[`Student`]**.
///
/// If fetching the data fails, then it will return a [`BlueArchiveError`], as the data would not have any **[`Students`][`Student`]**.
pub async fn fetch_random_student(
    language: &Language,
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
