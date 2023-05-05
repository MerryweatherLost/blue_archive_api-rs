use blue_archive::{enums::School, fetcher::StudentFetcher, types::students::Age, Language};

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // In this example, we will see two different ways of filtering.

    // We can fetch all students first on their own...
    // For this one, we will use Chinese.
    let students = blue_archive::fetch_all_students(&Language::Chinese).await?;

    // We will also create a fetcher for later use.
    let fetcher = StudentFetcher::new(&Language::English).await?;

    // And now, let us fetch 17 year olds under the Trinity General School.

    // First, let us try with the filter function.
    let trinity_17 = blue_archive::filter(&students)
        .apply(Age(Some(17)))
        .apply(School::Trinity)
        .finish();

    // And now, let us fetch the same thing using the student fetcher.
    // The implementations are relatively identical.
    let trinity_17 = (fetcher.filter())
        .apply(Age(Some(17)))
        .apply(School::Trinity)
        .finish();

    // Let us print out all the filtered students.
    println!(
        "|:: Age 17 & {} : [{}/{}]",
        School::Trinity.full_name(),
        trinity_17.len(),
        students.len(),
    );
    trinity_17.iter().for_each(|student| println!("{student}"));
    // There are way more implementations then shown. You can also implement your own using the StudentFilter if you want!
    Ok(())
}
