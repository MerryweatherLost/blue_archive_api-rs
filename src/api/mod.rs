pub mod enums;
pub mod errors;
pub mod fetcher;

use crate::api::enums::*;
use crate::enums::*;

use crate::types::*;
use anyhow::Result;
use rand::seq::SliceRandom;
use reqwest::Response;

use enums::StudentQuery;
use errors::BlueArchiveError;

pub(crate) mod helper {

    use super::*;

    /**
        Main function that provides a [`Response`] based on the provided [`Endpoints`] enum.
    */
    pub async fn fetch_response(endpoint: Endpoints) -> Result<Response, BlueArchiveError> {
        let response_string = match endpoint {
            Endpoints::Status => "".to_string(),
            Endpoints::Character(possible_char_query) => {
                let path = match possible_char_query {
                    Some(char_query) => match char_query {
                        StudentNameOrQuery::Name(string) => string, // Direct name of Character e.g. Asuna
                        StudentNameOrQuery::Query(query) => query.to_string(), // The specific query e.g. query?school=Abydos
                    },
                    // If empty, means that all instances of PartialStudent, or "character/" will be returned in a request.
                    None => "".to_string(),
                };
                format!("character/{}", path)
            }
            Endpoints::Equipment(id_or_string) => {
                let path = match id_or_string {
                    EquipmentIDOrName::ID(id) => format!("{}?id=true", id),
                    EquipmentIDOrName::Name(string) => string,
                };
                format!("equipment/{}", path)
            }
            Endpoints::Stage => todo!(),
            Endpoints::Raid => "raid".to_string(),
            Endpoints::Banner => "banner".to_string(),
        };
        Ok(reqwest::get(format!("https://api.ennead.cc/buruaka/{}", response_string)).await?)
    }

    /**
        When a query result, normally a list of [`String`] containing the names of the Students need to be converted to a [`Vec<Student>`].
    */
    pub async fn fetch_students_from_query_response(
        response: Response,
    ) -> Result<Vec<Student>, BlueArchiveError> {
        let mut students: Vec<Student> = vec![];
        let student_name_list = match response.json::<Vec<String>>().await {
            Ok(name_list) => name_list,
            Err(err) => return Err(BlueArchiveError::Reqwest(err)),
        };

        // Concurrent Requests
        let bodies =
            futures::future::join_all(student_name_list.into_iter().map(|name| async move {
                let response =
                    match reqwest::get(format!("https://api.ennead.cc/buruaka/character/{}", name))
                        .await
                    {
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
    Fetches a [`Student`] based on a given name.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_student_by_name("Asuna").await {
                Ok(student) => {
                    println!(
                        "Name: {}\nProfile:{}",
                        student.character.name, student.character.profile
                    )
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_student_by_name<IntoString: Into<String>>(
    name: IntoString,
) -> Result<Student, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Name(
        name.into(),
    ))))
    .await?;

    Ok(response.json::<Student>().await?)
}

/**
    Fetches a [`Student`] based on a given ID.
    ```
        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            match blue_archive::fetch_student_by_id(16001).await {
                Ok(student) => {
                    println!(
                        "Name: {}\nAge:{}, Club:{}",
                        student.character.name, student.info.age, student.info.club
                    )
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
            Ok(())
        }

*/
pub async fn fetch_student_by_id(id: u32) -> Result<Student, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::ID(id),
    ))))
    .await?;
    let id_student = response.json::<IDStudent>().await?;
    fetch_student_by_name(id_student.name).await
}

/**
    Fetches all instances of [`PartialStudent`] in a [`Vec`]. This uses the `character/` endpoint directly,
    so it is not that expensive as [`fetch_all_students`], but it has limited information. See [`PartialStudent`] for more information of the data.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_all_partial_students().await {
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
pub async fn fetch_all_partial_students() -> Result<Vec<PartialStudent>, BlueArchiveError> {
    let response = match helper::fetch_response(Endpoints::Character(None)).await {
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

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_all_students().await {
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
        }
    ```
*/
pub async fn fetch_all_students() -> Result<Vec<Student>, BlueArchiveError> {
    let mut students: Vec<Student> = vec![];
    let partial_students = fetch_all_partial_students().await?;

    // Concurrent Requests
    let bodies =
        futures::future::join_all(partial_students.into_iter().map(|partial| async move {
            let response = match reqwest::get(format!(
                "https://api.ennead.cc/buruaka/character/{}",
                partial.name
            ))
            .await
            {
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
    Fetches a random [`Student`].

    In the case randomization fails, [`BlueArchiveError::RandomError`] is presented.

    ## Examples

    ```
        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_random_student().await {
                Ok(student) => {
                    println!(
                        "Name: {}\nAge:{}, Club:{}",
                        student.character.name, student.info.age, student.info.club
                    )
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            };
        }
    ```
*/
pub async fn fetch_random_student() -> Result<Student, BlueArchiveError> {
    let partial_students = fetch_all_partial_students().await?;
    match partial_students.choose(&mut rand::thread_rng()) {
        Some(found) => Ok(fetch_student_by_name(&found.name).await?),
        None => Err(BlueArchiveError::RandomError),
    }
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`School`] enum.

    ## Examples

    ```
        use blue_archive::enums::School;

        #[tokio::main]
        async fn main() {
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
        }
    ```
*/
pub async fn fetch_students_by_school(school: School) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::School(school),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Role`] enum.

    ## Examples

    ```
        use blue_archive::enums::Role;

        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_students_by_role(Role::Attacker).await {
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
        }
    ```
*/
pub async fn fetch_students_by_role(role: Role) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Role(role),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Type`] enum.

    ## Examples

    ```
        use blue_archive::enums::Type;

        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_students_by_type(Type::Special).await {
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
        }
    ```
*/
pub async fn fetch_students_by_type(type_: Type) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Type(type_),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Weapon`] enum.

    ## Examples

    ```
        use blue_archive::Weapon;

        #[tokio::main]
        async fn main() {
            let assault_rifles = match blue_archive::fetch_students_by_weapon(Weapon::AR).await {
                Ok(students) => {
                    println!("Here is a list of students within the Assault Rifles Category:");
                    students
                }
                Err(err) => return println!("Unable to Retrieve Students! {err}"),
            };
            for student in assault_rifles.iter() {
                println!("{}, {}", student.character.name, student.info.age);
            }
        }
    ```
*/
pub async fn fetch_students_by_weapon(weapon: Weapon) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Weapon(weapon),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Position`] enum.

    ## Examples

    ```
        use blue_archive::Position;

        #[tokio::main]
        async fn main() {
            if let Ok(students) = blue_archive::fetch_students_by_position(Position::Front).await {
                for student in students.iter() {
                    println!("{student}")
                }
            }
        }
    ```
*/
pub async fn fetch_students_by_position(
    position: Position,
) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Position(position),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Damage`] enum.

    ## Examples

    ```
        use blue_archive::Damage;

        #[tokio::main]
        async fn main() {
            if let Ok(students) = blue_archive::fetch_students_by_damage(Damage::Explosion).await {
                for student in students.iter() {
                    println!("{student}")
                }
            }
        }
    ```
*/
pub async fn fetch_students_by_damage(damage: Damage) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Damage(damage),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Armor`] enum.

    ## Examples

    ```
        use blue_archive::Armor;

        #[tokio::main]
        async fn main() {
            if let Ok(students) = blue_archive::fetch_students_by_armor(Armor::Heavy).await {
                for student in students.iter() {
                    println!("{student}")
                }
            }
        }
    ```
*/
pub async fn fetch_students_by_armor(armor: Armor) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(Some(StudentNameOrQuery::Query(
        StudentQuery::Armor(armor),
    ))))
    .await?;
    helper::fetch_students_from_query_response(response).await
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
    let response = helper::fetch_response(Endpoints::Equipment(EquipmentIDOrName::ID(id))).await?;
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
pub async fn fetch_equipment_by_name<IntoString: Into<String>>(
    name: IntoString,
) -> Result<Equipment, BlueArchiveError> {
    let response =
        helper::fetch_response(Endpoints::Equipment(EquipmentIDOrName::Name(name.into()))).await?;
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
                        println!("{}, {}", banner.id, banner.started_at)
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
