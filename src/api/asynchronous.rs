use crate::api::enums::*;
use crate::enums::*;

use crate::types::*;
use crate::Endpoints;
use anyhow::Result;
use rand::seq::SliceRandom;
use reqwest::Response;

use super::enums::StudentQuery;
use super::errors::BlueArchiveError;

pub(crate) mod helper {

    use super::*;

    pub async fn fetch_response(endpoint: Endpoints) -> Result<Response, BlueArchiveError> {
        let response_string = match endpoint {
            Endpoints::Status => "".to_string(),
            Endpoints::Character(possible_char_query) => {
                let path = match possible_char_query {
                    Some(char_query) => match char_query {
                        CharacterNameOrQuery::Name(string) => string, // Direct name of Character e.g. Asuna
                        CharacterNameOrQuery::Query(query) => query.to_string(), // The specific query e.g. query?school=Abydos
                    },
                    // If empty, means that all instances of PartialStudent, or "character/" will be returned in a request.
                    None => "".to_string(),
                };
                format!("character/{}", path)
            }
            Endpoints::Equipment(id_or_string) => {
                let path = match id_or_string {
                    EquipmentIDOrString::ID(id) => format!("{}?id=true", id),
                    EquipmentIDOrString::String(string) => format!("{}", string),
                };
                format!("equipment/{}", path)
            }
            Endpoints::Stage => todo!(),
            Endpoints::Raid => todo!(),
            Endpoints::Banner => todo!(),
        };
        match reqwest::get(format!("https://api.ennead.cc/buruaka/{}", response_string)).await {
            Ok(response) => Ok(response),
            Err(err) => {
                return Err(BlueArchiveError::RequestError(err));
            }
        }
    }

    /**
        When a query result, normally a list of [`String`] containing the names of the Student's need to be converted to a [`Vec<Student>`]
    */
    pub async fn fetch_students_from_query_response(
        response: Response,
    ) -> Result<Vec<Student>, BlueArchiveError> {
        let mut students: Vec<Student> = vec![];
        let student_name_list = match response.json::<Vec<String>>().await {
            Ok(name_list) => name_list,
            Err(err) => return Err(BlueArchiveError::DeserializationError(err)),
        };

        // Concurrent Requests
        let bodies = futures::future::join_all(
            student_name_list
                .into_iter()
                .map(|name| async move { fetch_student_by_name(name).await }),
        )
        .await;
        for body in bodies {
            match body {
                Ok(student) => students.push(student),
                Err(ex) => return Err(ex),
            }
        }

        Ok(students)
    }
}

/**
Fetches the current status of the API in a [`APIStatus`] struct.

Contains status code information, uptime results and more.
*/
pub async fn fetch_status() -> Result<APIStatus, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Status).await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };
    match response.json::<APIStatus>().await {
        Ok(status) => Ok(status),
        Err(err) => Err(BlueArchiveError::DeserializationError(err)),
    }
}

pub async fn fetch_student_by_name<IntoStr: Into<String>>(
    name: IntoStr,
) -> Result<Student, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Character(Some(
        CharacterNameOrQuery::Name(name.into()),
    )))
    .await
    {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };

    match response.json::<Student>().await {
        Ok(student) => Ok(student),
        Err(err) => Err(BlueArchiveError::DeserializationError(err)),
    }
}

pub async fn fetch_student_by_id(id: u32) -> Result<Student, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Character(Some(
        CharacterNameOrQuery::Query(StudentQuery::ID(id)),
    )))
    .await
    {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };
    match response.json::<IDStudent>().await {
        Ok(id_student) => match fetch_student_by_name(id_student.name).await {
            Ok(student) => Ok(student),
            Err(ex) => Err(ex),
        },
        Err(err) => Err(BlueArchiveError::DeserializationError(err)),
    }
}

pub async fn fetch_all_partial_students() -> Result<Vec<PartialStudent>, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Character(None)).await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };
    match response.json::<PartialStudentData>().await {
        Ok(partial) => Ok(partial.data),
        Err(err) => Err(BlueArchiveError::DeserializationError(err)),
    }
}

pub async fn fetch_all_students() -> Result<Vec<Student>, BlueArchiveError> {
    let mut students: Vec<Student> = vec![];
    let partial_students = fetch_all_partial_students().await?;

    // Concurrent Requests
    let bodies = futures::future::join_all(
        partial_students
            .into_iter()
            .map(|partial| async move { fetch_student_by_name(partial.name).await }),
    )
    .await;
    for body in bodies {
        match body {
            Ok(student) => students.push(student),
            Err(ex) => return Err(ex),
        }
    }
    Ok(students)
}

pub async fn fetch_random_student() -> Result<Student, BlueArchiveError> {
    let partial_students = fetch_all_partial_students().await?;
    match partial_students.choose(&mut rand::thread_rng()) {
        Some(found) => return Ok(fetch_student_by_name(&found.name).await?),
        None => Err(BlueArchiveError::RandomError),
    }
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`School`] enum.

    ```
        use blue_archive::enums::School;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            match blue_archive::fetch_students_by_school(School::Hyakkiyako).await {
                Ok(students) => {
                    for student in students.iter() {
                        println!(
                            "Name: {}\nAge:{}, Club:{}",
                            student.character.name, student.info.age, student.info.club
                        )
                    }
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
            Ok(())
        }
    ```
*/
pub async fn fetch_students_by_school(school: School) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(CharacterNameOrQuery::Query(
        StudentQuery::School(school),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

pub async fn fetch_equipment_by_id(id: u32) -> Result<Equipment, BlueArchiveError> {
    let response =
        helper::fetch_response(Endpoints::Equipment(EquipmentIDOrString::ID(id))).await?;
    match response.json::<Equipment>().await {
        Ok(equipment) => Ok(equipment),
        Err(err) => Err(BlueArchiveError::DeserializationError(err)),
    }
}
