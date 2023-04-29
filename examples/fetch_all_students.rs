use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    for student in students {
        println!("{student} : {:?}", student.image)
    }
    Ok(())
}
