use anyhow::{Context, Result};
use blue_archive::BlueArchiveFetcher;

#[tokio::main]
async fn main() -> Result<()> {
    // Preliminary Setup
    let fetcher = BlueArchiveFetcher::new().await?;

    // -> Asuna's age is 17, and will be reflected as Some(17).
    println!("Fetching Asuna's Age!");
    let asuna_age = fetcher
        .get_student_by_name("Asuna")
        .context("Failed to find Asuna!")?
        .age();
    println!("Asuna's Age: {asuna_age}");

    println!();

    // -> Although Shun is Top Secret, & will be reflected as None.
    println!("Fetching Shun's Age!");
    let shun_age = fetcher
        .get_student_by_name("Shun")
        .context("Failed to find Shun!")?
        .age();
    println!("Shun's Age: {shun_age}");

    // -> Now, let's iterate over all of the students.
    println!("<| Iterating Over [{}] Students |>", fetcher.students.len());
    for student in &fetcher.students {
        println!("{} :->: {}", student.name(), student.age())
    }
    Ok(())
}
