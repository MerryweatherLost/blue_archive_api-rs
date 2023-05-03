use blue_archive::enums::Club;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let students =
        blue_archive::fetch_all_students_without_extra(&blue_archive::Language::English).await?;

    let max = Club::iter()
        .map(|club| club.to_string().len())
        .max()
        .unwrap();

    println!("{max:?}");

    Club::iter().for_each(|club| {
        println!("|::{club}{}::|", " ".repeat(max - club.to_string().len()));
        students.iter().for_each(|student| {
            if student.club() == club {
                println!("{student}")
            }
        })
    });

    Ok(())
}
