use blue_archive::{Club, Region};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnc_students =
        blue_archive::fetch_students_by_club(Club::CleaningAndClearing, Some(Region::Japan))
            .await?;

    println!(
        "Students apart of Cleaning & Clearing: [{}]",
        cnc_students.len()
    );
    for student in &cnc_students {
        println!("Details:[{student}] : Position:[{}]", student.position())
    }
    Ok(())
}
