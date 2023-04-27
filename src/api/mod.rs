use crate::enums::Language;
use crate::filter::StudentFilterOptions;
use crate::types::Student;
use crate::{BlueArchiveError, DATA_URI};

use rand::seq::IteratorRandom;
use reqwest::Client;
pub use reqwest::{Request, Response, StatusCode};

use anyhow::Result;
use strum_macros::Display;

/// Contains the endpoints for the data, they mainly just represent the path of what data is obtained.
#[derive(Debug, Display)]
pub(crate) enum Endpoint {
    Currency,
    Enemies,
    Equipment,
    Furniture,
    Items,
    Localization,
    Raids,
    Students,
    Summons,
}

/// TBD
pub(crate) mod internal {
    use super::*;

    /// Fetches a response using a given endpoint.
    pub(crate) async fn fetch_response(
        endpoint: &Endpoint,
        language: &Language,
    ) -> Result<Response, BlueArchiveError> {
        let url = format!(
            "{}/{}/{}.json",
            DATA_URI,
            language.id(),
            endpoint.to_string().to_lowercase()
        );
        Ok(reqwest::get(url).await?.error_for_status()?)
    }

    /// Obtains all Students without adding extra content, useful for filtered [`Student`]'s.
    pub(crate) async fn fetch_all_students_without_extra(
        language: &Language,
    ) -> Result<Vec<Student>, BlueArchiveError> {
        let response = internal::fetch_response(&Endpoint::Students, language).await?;
        Ok(response.json::<Vec<Student>>().await?)
    }
}

/// TBD
pub async fn fetch_all_students(language: &Language) -> Result<Vec<Student>, BlueArchiveError> {
    let mut students = internal::fetch_all_students_without_extra(language).await?;
    futures::future::join_all(
        students
            .iter_mut()
            .map(|student| async move { student.fetch_extra_data(&Client::new()).await }),
    )
    .await;
    Ok(students)
}

/// TBD
pub async fn fetch_student_by_name(
    name: impl Into<String>,
    language: &Language,
) -> Result<Option<Student>, BlueArchiveError> {
    let name: String = name.into();
    let possible_student = internal::fetch_all_students_without_extra(language)
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

pub async fn fetch_random_student(language: &Language) -> Result<Student, BlueArchiveError> {
    Ok(fetch_all_students(language)
        .await?
        .into_iter()
        .choose(&mut rand::thread_rng())
        .expect("failed to randomize students!"))
}

pub fn filter(students: &Vec<Student>) -> StudentFilterOptions {
    StudentFilterOptions::new(students)
}
