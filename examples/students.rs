use blue_archive::{Query, School, SquadType};

#[tokio::main]
async fn main() {
    match blue_archive::fetch_students_by_queries([
        Query::SquadType(SquadType::Striker),
        Query::School(School::Trinity),
    ])
    .await
    {
        Ok(students) => {
            println!("Here is a list of Blue Archive Students that are Strikers & apart of Trinity General School.");
            for student in students.iter() {
                println!("-----------------------------------------------------------");
                println!(
                    "Name: {}\nAge: {}\nSchool/School Year: {} / {}\nClub: {}\nBase Stars: {}",
                    student.character.name,
                    student.info.age,
                    student.info.school,
                    student.info.school_year,
                    student.info.club,
                    "⭐".repeat(student.character.base_star as usize)
                );
            }
        }
        Err(err) => println!("Failed to Obtain Students!\n{err}",),
    };
}
