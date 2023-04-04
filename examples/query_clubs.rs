use blue_archive::Club;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnc_students = blue_archive::fetch_students_by_club(Club::CleaningAndClearing).await?;
    for student in &cnc_students {
        println!("{student} : is apart of {}", student.club())
    }
    Ok(())
}
