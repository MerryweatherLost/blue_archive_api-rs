use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    for student in students {
        println!("{}", student.full_name_with_last())
    }

    let student = blue_archive::fetch_student_by_name("Wakamo", &Language::English)
        .await?
        .expect("shit!");
    println!(
        "{:?}",
        student.image().alternative_portrait().await.unwrap()
    );
    Ok(())
}
