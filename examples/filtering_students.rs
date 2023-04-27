use blue_archive::{enums::School, types::students::Age, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    let trinity_17 = blue_archive::filter(&students)
        .apply(Age(Some(17)))
        .apply(School::Trinity)
        .finish();
    println!(
        "(All <-> Age 17 & Trinity) : ({} <-> {})",
        students.len(),
        trinity_17.len()
    );
    Ok(())
}
