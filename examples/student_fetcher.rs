use std::time::Instant;

use blue_archive::{School, Squad};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut start = Instant::now();
    let student_fetcher = blue_archive::BlueArchiveFetcher::new().await?;
    println!("StudentFetcher :: {:?}", Instant::now() - start);

    start = Instant::now();
    match student_fetcher.get_student_by_name("Asuna") {
        Some(asuna) => println!("{asuna}"),
        None => panic!(),
    }
    println!("Process :: {:?}", Instant::now() - start);

    start = Instant::now();
    blue_archive::fetch_student_by_name("Asuna").await?;
    println!("Partial Students :: {:?}", Instant::now() - start);

    start = Instant::now();
    let trinity_students = student_fetcher
        .filter()
        .apply(School::Trinity)
        .apply(Squad::Striker)
        .finish_ref();
    println!(
        "Filter Trinity Students [Strikers] :: [{}/{}]:: {:?}",
        trinity_students.len(),
        student_fetcher.students.len(),
        Instant::now() - start
    );
    Ok(())
}
