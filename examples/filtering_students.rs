use blue_archive::{enums::School, fetcher::StudentFetcher, types::students::Age, Language};

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can filter students through this way...
    let students = blue_archive::fetch_all_students(&Language::English).await?;
    let trinity_17 = blue_archive::filter(&students)
        .apply(Age(Some(17)))
        .apply(School::Trinity)
        .finish();

    // Or even by using the fetcher.
    let fetcher = StudentFetcher::new(&Language::English).await?;
    let trinity_17 = fetcher
        .filter()
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
