use crate::api::enums::*;
use crate::{enums::*, helper};

use crate::errors::BlueArchiveError;
use crate::types::*;
use anyhow::Result;

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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::School(school),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::Role(role),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::SquadType(squad),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::Weapon(weapon),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::Position(position),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::Damage(damage),
    ]))))
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
    let response = helper::fetch_response(Endpoints::Character(Some(QueryKind::Query(vec![
        StudentQuery::Armor(armor),
    ]))))
    .await?;
    helper::fetch_students_from_query_response(response).await
}
