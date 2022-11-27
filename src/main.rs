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
