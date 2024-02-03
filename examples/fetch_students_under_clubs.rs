use blue_archive::{enums::Club, Language};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Just calculating the club with the longest name...
    let max = Club::iter()
        .map(|club| club.to_string().len())
        .max()
        .unwrap();

    // Let's now fetch some students.
    let students = blue_archive::fetch_all_students(Language::English).await?;

    // We will iterate through every club (thanks strum macros!), and perform certain behavior to print out each student in a specific club.
    Club::iter().for_each(|club| {
        // Formatting and justifying spaces to the right!
        println!("|::{club}{}::|", " ".repeat(max - club.to_string().len()));
        // Iterating over each student and checking if they associate with the club.
        students.iter().for_each(|student| {
            (student.club() == club).then(|| println!("{student}"));
        })
    });

    // Of course, you can just filter the students through the direct filter.
    let maids = blue_archive::filter(&students)
        .apply(Club::CleaningAndClearing)
        .finish();
    println!("{} Members : [{}]", Club::CleaningAndClearing, maids.len());
    maids.iter().for_each(|student| println!("{student}"));

    Ok(())
}
