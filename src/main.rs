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
