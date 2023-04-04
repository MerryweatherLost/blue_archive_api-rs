#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let all_students = blue_archive::fetch_all_students().await?;
    for student in &all_students {
        println!(
            "{student} | apart of {} | {}:{}",
            student.club(),
            student.rarity(),
            "⭐".repeat(student.character.base_star as usize)
        )
    }
    Ok(())
}
