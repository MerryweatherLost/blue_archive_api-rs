use blue_archive::{types::students::Age, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    let new_students = blue_archive::filter(&students)
        .apply(Age(Some(17)))
        .finish();
    println!(
        "(All <-> Age 17) : ({} <-> {})",
        students.len(),
        new_students.len()
    );
    Ok(())
}
