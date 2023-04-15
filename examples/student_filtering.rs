use anyhow::Result;
use blue_archive::{Query, Region, School, Squad};

#[allow(unused_assignments)]
#[tokio::main]
async fn main() -> Result<()> {
    // You are able to filter Students through either direct querying:
    let mut students = blue_archive::fetch_students_by_queries([
        Query::Squad(Squad::Striker),
        Query::School(School::Trinity),
        Query::Region(Region::Japan),
    ])
    .await?;

    // or, you can opt in through fetching all students, and using a filter builder-like pattern.
    // this is more expensive then querying before, which only pulls the names returned from the API and fetches their full data.
    // - finish_ref will allow you to have a returned reference of each Student.
    // - while finish clones the Students and returns a Vec<Student>.
    students = blue_archive::filter(&blue_archive::fetch_all_students(Some(Region::Japan)).await?)
        .apply(School::Trinity)
        .apply(Squad::Striker)
        .finish();

    println!("Here is a list of Blue Archive Students that are Strikers & apart of Trinity General School.");
    for student in students.iter() {
        println!("-----------------------------------------------------------");
        println!(
            "{student} :: {}",
            "⭐".repeat(student.character.base_star as usize)
        );
    }
    Ok(())
}
