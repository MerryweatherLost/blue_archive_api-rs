use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    for student in students {
        println!(
            "{student} : Full Body: [{:?}]",
            student.image.portrait.full_body_url
        )
    }
    Ok(())
}
