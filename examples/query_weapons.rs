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

    // You can also do this with the filter:
    let students = blue_archive::fetch_all_students(None).await?;
    let sniper_rifle_students = blue_archive::filter(&students)
        .apply(Weapon::SR)
        .finish_ref();
    println!(
        "Amount of students using sniper rifles: {}",
        sniper_rifle_students.len()
    );
    for student in &sniper_rifle_students {
        println!("({})::[{}]", student, student.weapon().full_name())
    }
    Ok(())
}
