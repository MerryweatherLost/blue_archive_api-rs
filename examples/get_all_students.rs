use anyhow::Result;
use blue_archive::Language;

#[cfg(feature = "blocking")]
pub fn main() -> Result<()> {
    // You can obtain the students in a blocking context now.
    let students = blue_archive::blocking::get_all_students(Language::English)?;
    println!("# of students: {}", students.len());
    // You may also use filtering and the StudentFetcher as you would usually do, though with blocking you will have to use the `new_blocking` function.
    let fetched = blue_archive::StudentFetcher::new_blocking(Language::Japanese)?;
    if let Some(student) = fetched.get_random_student() {
        println!("Randomized Student: [{student}]")
    }
    Ok(())
}
