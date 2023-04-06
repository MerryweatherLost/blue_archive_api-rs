//! Contains the bulk of the wrapper's implementations.
//!
//! Part of it is already used in the crate's library.

pub mod enums;
pub mod errors;
pub mod fetcher;

#[cfg(feature = "query")]
pub mod query;

#[cfg(feature = "query")]
pub use query::*;

use crate::api::enums::*;

use crate::{types::*, Region};
use anyhow::Result;
use rand::seq::SliceRandom;
use reqwest::Response;

use enums::Query;
use errors::BlueArchiveError;

pub mod filter;

pub use crate::API_URI;

use self::enums::students::StudentQueryBuilder;

pub(crate) mod helper {

    use super::*;

    /**
        Main function that provides a [`Response`] based on the provided [`Endpoints`] enum.
    */
    pub(crate) async fn fetch_response(endpoint: Endpoints) -> Result<Response, BlueArchiveError> {
        let response_string = match endpoint {
            Endpoints::Status => "".to_string(),
            Endpoints::Character(query) => {
                format!("character/{query}")
            }
            Endpoints::Equipment(id_or_string) => {
                let path = match id_or_string {
                    IDOrName::ID(id) => format!("{}?id=true", id),
                    IDOrName::Name(string) => string,
                };
                format!("equipment/{}", path)
            }
            Endpoints::_Stage => "stage".to_string(),
            Endpoints::Raid => "raid".to_string(),
            Endpoints::Banner => "banner".to_string(),
        };
        Ok(reqwest::get(format!("{}/{}", API_URI, response_string)).await?)
    }

    /**
        When a query result, normally a list of [`String`] containing the names of the Students need to be converted to a [`Vec<Student>`].
    */
    pub(crate) async fn fetch_students_from_query_response(
        response: Response,
    ) -> Result<Vec<Student>, BlueArchiveError> {
        let mut students: Vec<Student> = vec![];
        let student_name_list = match response.json::<Vec<String>>().await {
            Ok(name_list) => name_list,
            Err(_) => vec![], // <- represents no students.
        };

        // Concurrent Requests
        let bodies =
            futures::future::join_all(student_name_list.into_iter().map(|name| async move {
                let response = match reqwest::get(format!("{}/character/{}", API_URI, name)).await {
                    Ok(res) => res,
                    Err(err) => return Err(BlueArchiveError::Reqwest(err)),
                };
                Ok(response.json::<Student>().await?)
            }))
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

## Examples

```
    use reqwest::StatusCode;

    #[tokio::main]
    async fn main() {
        match blue_archive::fetch_status().await {
            Ok(status) => {
                match StatusCode::from_u16(status.code) {
                    Ok(code) => println!("{code}"),
                    Err(err) => println!("{err}"),
                };
            }
            Err(err) => println!("Unable to fetch Status! {err}"),
        }
    }
```

*/
pub async fn fetch_status() -> Result<APIStatus, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Status).await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };
    Ok(response.json::<APIStatus>().await?)
}

/**
    Fetches a [`Student`] based on a given name & region.
    If region is [`None`], then it will default to **Global**.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_student_by_name("Asuna", None).await {
                Ok(student) => {
                    println!(
                        "Name: {}\nProfile:{}",
                        student.name(), student.character.profile
                    )
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_student_by_name<IS: Into<String>>(
    name: IS,
    region: Option<Region>,
) -> Result<Student, BlueArchiveError> {
    let name: String = name.into();
    let query_string = StudentQueryBuilder::new().build_with_student_name(name.clone(), region);
    println!("{query_string:#?}");
    let response = helper::fetch_response(Endpoints::Character(query_string)).await?;

    Ok(response.json::<Student>().await?)
}

/**
    Fetches a [`Student`] based on a given ID.

    ## Examples

    ```
        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            match blue_archive::fetch_student_by_id(16001).await {
                Ok(student) => {
                    println!(
                        "Name: {}\nAge:{}, Club:{}",
                        student.name(), student.age(), student.club()
                    )
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
            Ok(())
        }
    ```
*/
pub async fn fetch_student_by_id(id: u32) -> Result<Student, BlueArchiveError> {
    let student_query = StudentQueryBuilder::new().build_with_single(Query::ID(id));
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
    fetch_student_by_name(response.json::<IDStudent>().await?.name, None).await
}
/**
    Fetches a [`Vec`] of [`Student`] based on a given set of queries.

    ## Examples

    ```
        use blue_archive::{School, Squad, Query};

        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_students_by_queries([
                Query::Squad(Squad::Striker),
                Query::School(School::Trinity),
            ])
            .await
            {
                Ok(students) => {
                    println!("Here is a list of Blue Archive Students that are Strikers & apart of Trinity General School.");
                    for student in students.iter() {
                        println!("{student}");
                    }
                }
                Err(err) => println!("Failed to Obtain Students!\n{err}",),
            };
        }
    ```
*/
pub async fn fetch_students_by_queries<Q: Into<Vec<Query>>>(
    queries: Q,
) -> Result<Vec<Student>, BlueArchiveError> {
    let student_query = StudentQueryBuilder::new().build_with_multiple(queries.into());
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
    helper::fetch_students_from_query_response(response).await
}
/**
    Fetches all instances of [`PartialStudent`] in a [`Vec`]. This uses the `character/` endpoint directly,
    so it is not that expensive as [`fetch_all_students`], but it has limited information. See [`PartialStudent`] for more information of the data.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_all_partial_students(None).await {
                Ok(partial_students) => {
                    for student in partial_students.iter() {
                        println!("Name: {}\nProfile:{}", student.name, student.profile)
                    }
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_all_partial_students(
    region: Option<Region>,
) -> Result<Vec<PartialStudent>, BlueArchiveError> {
    let student_query =
        StudentQueryBuilder::new().build_with_single(Query::Region(region.unwrap_or_default()));
    let response = match helper::fetch_response(Endpoints::Character(student_query)).await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };
    Ok(response.json::<PartialStudentData>().await?.data)
}

/**
    Attempts to fetch all instances of [`Student`] in a [`Vec`] through future chaining,
    since the `character/` endpoint at this time only has instances of [`PartialStudent`] data.

    This is a workaround for that issue.

    **Unfortunately, this function has to do a number of API calls, which is more expensive.**

    There is a problem where I am unable to search for the specific Japanese names in the API, so I have to use English names in order to not fail.

    [`Region`] will default to [`Region::Global`] if [`None`].

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_all_students(None).await {
                Ok(students) => {
                    for student in students.iter() {
                        println!("{student}")
                    }
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_all_students(region: Option<Region>) -> Result<Vec<Student>, BlueArchiveError> {
    let mut students: Vec<Student> = vec![];
    let region = region.unwrap_or_default();
    let partial_students = fetch_all_partial_students(None).await?;

    // Concurrent Requests
    let bodies =
        futures::future::join_all(partial_students.into_iter().map(|partial| async move {
            let uri = format!("{}/character/{}?{}", API_URI, partial.name, region);
            let response = match reqwest::get(&uri).await {
                Ok(res) => res,
                Err(err) => return Err(BlueArchiveError::Reqwest(err)),
            };
            Ok(response.json::<Student>().await)
        }))
        .await;
    for body in bodies {
        students.push(body??)
    }
    Ok(students)
}

/**
    Fetches a random [`Student`], though can return [`None`] if the `students` it is iterating over are empty.

    [`Region`] will default to [`Region::Global`] if [`None`].

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_random_student(None).await {
                Ok(possible_student) => {
                    match possible_student {
                        Some(student) => {
                            println!("{student}")
                        }
                        None => {
                            println!("Could not randomize students.")
                        }
                    }
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_random_student(
    region: Option<Region>,
) -> Result<Option<Student>, BlueArchiveError> {
    let partial_students = fetch_all_partial_students(region).await?;
    match partial_students.choose(&mut rand::thread_rng()) {
        Some(found) => Ok(Some(fetch_student_by_name(&found.name, None).await?)),
        None => Ok(None),
    }
}

/**
    Fetches [`Equipment`] based on a given ID.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_equipment_by_id(6000).await {
                Ok(equipment) => println!("{}", equipment.drops[0].stage_name),
                Err(err) => println!("{}", err),
            }
        }
    ```
*/
pub async fn fetch_equipment_by_id(id: u32) -> Result<Equipment, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Equipment(IDOrName::ID(id))).await?;
    Ok(response.json::<Equipment>().await?)
}

/**
    Fetches [`Equipment`] based on a given name.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_equipment_by_name("t1 hairpin").await {
                Ok(equipment) => println!("{}", equipment.data.id),
                Err(err) => println!("{}", err),
            }
        }
    ```
*/
pub async fn fetch_equipment_by_name<IS: Into<String>>(
    name: IS,
) -> Result<Equipment, BlueArchiveError> {
    let response =
        helper::fetch_response(Endpoints::Equipment(IDOrName::Name(name.into()))).await?;
    Ok(response.json::<Equipment>().await?)
}
/**
    Fetches [`Raids`] from the API.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_raids().await {
                Ok(raids) => {
                    for raid in raids.ended {
                        let start_time = match raid.start_time() {
                            Ok(time) => time.to_string(),
                            Err(_) => "N/A".to_string(),
                        };
                        println!("Boss: {}, {}", raid.boss_name, start_time)
                    }
                }
                Err(err) => println!("Could not fetch raids!\n{}", err),
            }
        }
    ```
*/
pub async fn fetch_raids() -> Result<Raids, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Raid).await?;
    Ok(response.json::<Raids>().await?)
}

/**
    Fetches [`Banners`] from the API.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_banners().await {
                Ok(banners) => {
                    for banner in banners.ended.iter() {
                        println!("{banner}")
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
    ```
*/
pub async fn fetch_banners() -> Result<Banners, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Banner).await?;
    Ok(response.json::<Banners>().await?)
}
