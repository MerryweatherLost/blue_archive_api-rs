use crate::api::enums::*;
use crate::filter::StudentFilterOptions;
use crate::{enums::*, fetch_all_students, helper};

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
    Fetches a [`Vec`] of [`Student`] from a given [`Squad`] enum.

    ## Examples

    ```
        use blue_archive::enums::Squad;

        #[tokio::main]
        async fn main() {
            match blue_archive::fetch_students_by_squad_type(Squad::Special).await {
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
pub async fn fetch_students_by_squad_type(squad: Squad) -> Result<Vec<Student>, BlueArchiveError> {
    let response = helper::fetch_response(Endpoints::Character(
        StudentQueryBuilder::new().build_with_single(Query::Squad(squad)),
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
        use blue_archive::Club;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            let cnc_students = blue_archive::fetch_students_by_club(Club::CleaningAndClearing).await?;
            for student in &cnc_students {
                println!("{student} : is apart of {}", student.club())
            }
            Ok(())
        }
    ```
*/
pub async fn fetch_students_by_club(club: Club) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(StudentFilterOptions::new(&fetch_all_students().await?)
        .apply(club)
        .finish())
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Released`] struct.

    Note that this behaves a bit differently compared to other queries, as we will have to use [`fetch_all_students`].
    This will be more of an expensive call.

    ## Examples

    ```
        use blue_archive::Released;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            let released_students = blue_archive::fetch_students_by_released(Released(true)).await?;
            for student in &released_students {
                println!("{student} : is apart of {}", student.club())
            }
            Ok(())
        }
    ```
*/
pub async fn fetch_students_by_released(
    released: Released,
) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(StudentFilterOptions::new(&fetch_all_students().await?)
        .apply(released)
        .finish())
}

/**
    Fetches a [`Vec`] of [`Student`] from a given [`Age`] struct.

    Note that this behaves a bit differently compared to other queries, as we will have to use [`fetch_all_students`].
    This will be more of an expensive call.

    ## Examples

    ```
        use blue_archive::Age;

        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            let age_17_students = blue_archive::fetch_students_by_age(Age::from(17)).await?;
            for student in &age_17_students {
                println!("{student} : is apart of {}", student.club())
            }
            Ok(())
        }
    ```
*/
pub async fn fetch_students_by_age(age: Age) -> Result<Vec<Student>, BlueArchiveError> {
    Ok(StudentFilterOptions::new(&fetch_all_students().await?)
        .apply(age)
        .finish())
}

/// Returns a filter that can be used upon a [`Vec`] of [`Student`].
pub fn filter(students: &Vec<Student>) -> StudentFilterOptions {
    StudentFilterOptions::new(students)
}
