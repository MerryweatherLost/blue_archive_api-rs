use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Let us fetch some students. This time, we will use the Korean Language enum.
    let students = blue_archive::fetch_all_students(&Language::Korean).await?;
    // Time to iterate over each student and print their displayed implementation!
    students.iter().for_each(|student| println!("{student}"));
    Ok(())
}
