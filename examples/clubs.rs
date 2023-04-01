use std::collections::HashMap;

use anyhow::Result;
use blue_archive::{types::Student, Club};

use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut club_map = HashMap::<Club, Vec<&Student>>::new();
    let students = blue_archive::fetch_all_students().await?;

    for club in Club::iter() {
        let club_students: Vec<&Student> = students
            .iter()
            .filter(|student| student.club() == club)
            .collect();
        club_map.insert(club, club_students);
    }

    for (club, club_students) in club_map {
        println!("|> {club} <|",);
        for club_student in club_students {
            println!("{}", club_student.character.name)
        }
    }
    Ok(())
}
