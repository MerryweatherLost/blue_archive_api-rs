use crate::api::enums::*;
use crate::{enums::*, fetch_all_partial_students, fetch_all_students, helper};

use crate::errors::BlueArchiveError;
use crate::types::*;
use anyhow::Result;

use super::enums::students::StudentQueryBuilder;

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
    let response = helper::fetch_response(Endpoints::Character(
        StudentQueryBuilder::new().build_with_single(Query::School(school)),
    ))
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
    let response = helper::fetch_response(Endpoints::Character(
        StudentQueryBuilder::new().build_with_single(Query::Role(role)),
    ))
    .await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`SquadType`] enum.

    ## Examples

    ```
        use blue_archive::enums::SquadType;

        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_students_by_squad_type(SquadType::Special).await {
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
pub async fn fetch_students_by_squad_type(
    squad: SquadType,
) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(
        StudentQueryBuilder::new().build_with_single(Query::SquadType(squad)),
    ))
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
    let student_query = StudentQueryBuilder::new().build_with_single(Query::Weapon(weapon));
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
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
    let student_query = StudentQueryBuilder::new().build_with_single(Query::Position(position));
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
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
    let student_query = StudentQueryBuilder::new().build_with_single(Query::Damage(damage));
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
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
    let student_query = StudentQueryBuilder::new().build_with_single(Query::Armor(armor));
    let response = helper::fetch_response(Endpoints::Character(student_query)).await?;
    helper::fetch_students_from_query_response(response).await
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Club`] enum.

    Note that this behaves a bit differently compared to other queries, as we will have to use [`fetch_all_students`].
    This will be more of an expensive call.

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
pub async fn fetch_students_by_club(club: Club) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(fetch_all_students()
        .await?
        .into_iter()
        .filter(|student| student.club() == club)
        .collect::<Vec<Student>>())
}
