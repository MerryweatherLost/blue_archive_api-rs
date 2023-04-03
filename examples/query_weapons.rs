use blue_archive::Weapon;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match blue_archive::fetch_students_by_weapon(Weapon::SG).await {
        Ok(students) => {
            println!("Amount of students using shotguns: {}", students.len());
            for student in &students {
                println!("({})::[{}]", student, student.rarity().full_name())
            }
        }
        Err(why) => println!("{why:?}"),
    };
    Ok(())
}
