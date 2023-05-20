use blue_archive::{Language, School, StudentFetcher, WeaponType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // We use the fetcher as shown.
    // We create one using the Japanese Language enum, which will allow us to have japanese data.
    let fetcher = StudentFetcher::new(Language::Japanese).await?;

    // Now, we will filter students based on their school and type of weapon.
    // In this case, let's get students from Gehenna Academy that have the Sniper Rifle type of weapon.
    let gehenna_snipers = fetcher
        .filter()
        .apply(School::Gehenna)
        .apply(WeaponType::SR)
        .finish();

    // Now we will display them all.
    println!(
        "::| Snipers of {} : [{}/{}]",
        School::Gehenna.full_name(),
        gehenna_snipers.len(),
        fetcher.students.len()
    );
    gehenna_snipers.iter().for_each(|s| println!("{s}"));

    println!();

    // We can also fetch a specific student.
    // Let us try and get Aru (アル).
    let aru = fetcher.get_student_by_name("アル").unwrap();
    println!(
        r#"Aru (アル):
            - id: {}
            - age: {}
            - club: {}
            - full name (surname): {}
        "#,
        aru.id,
        aru.age(),
        aru.club(),
        aru.full_name_last()
    );

    // We can also randomize students to get a single one.
    let random_student = fetcher.get_random_student().unwrap();
    // I wonder who it will be this time?
    println!(
        "The random student of this second is: {}!",
        random_student.full_name_last()
    );

    // We can also get a random amount of students.
    let random_students = fetcher.get_random_students(5);
    println!(
        "Names of [5] Randomized Students: ({})",
        random_students
            .iter()
            .map(|s| s.full_name_last())
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(())
}
