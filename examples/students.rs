use blue_archive::{School, Weapon};

#[tokio::main]
async fn main() {
    let hyakkiyako_students = match blue_archive::fetch_students_by_school(School::Hyakkiyako).await
    {
        Ok(students) => {
            println!("Here is a list of students from the Hyakkiyako Alliance Academy:");
            students
        }
        Err(err) => return println!("Unable to Retrieve Students! {err}"),
    };
    for student in hyakkiyako_students.iter() {
        println!("{}, {}", student.character.name, student.info.age);
    }

    println!();

    let assault_rifles = match blue_archive::fetch_students_by_weapon(Weapon::AR).await {
        Ok(students) => {
            println!(
                "Here is a list of students within the {} Category:",
                Weapon::AR.full_name()
            );
            students
        }
        Err(err) => return println!("Unable to Retrieve Students! {err}"),
    };
    for student in assault_rifles.iter() {
        println!("{}, {}", student.character.name, student.info.age);
    }
}
