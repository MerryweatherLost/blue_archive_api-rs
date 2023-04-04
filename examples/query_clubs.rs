use blue_archive::Club;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let anzenkyoku_students = blue_archive::fetch_students_by_club(Club::Anzenkyoku).await?;
    Ok(())
}
