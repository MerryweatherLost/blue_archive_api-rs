use std::time::Instant;

use blue_archive::{School, Squad};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut start = Instant::now();
    // The student fetcher is created as so.
    let student_fetcher = blue_archive::StudentFetcher::new(None).await?;
    println!("StudentFetcher :: {:?}", Instant::now() - start);

    start = Instant::now();

    // You can use this to get specific Students.
    match student_fetcher.get_student_by_name("Asuna") {
        Some(asuna) => println!("{asuna}"),
        None => panic!("example cannot be completed."),
    }
    println!("Process :: {:?}", Instant::now() - start);

    // Just a test to run against regular fetching and the student fetcher.
    start = Instant::now();
    blue_archive::fetch_student_by_name("Asuna", None).await?;
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
