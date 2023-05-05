use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // First, let's try and fetch one student. We will use Hina for this example.
    // We can do this through simply calling the below crate function.
    let hina = (blue_archive::fetch_student_by_name("Hina", &Language::English).await?).unwrap();

    // Let us print some important details of Hina!
    let header = format!("|:: {} : Details", hina.full_name_last());
    println!("{header}");
    println!("{}", "-".repeat(header.len()));
    let segments = [
        ("age", format!("{}", hina.age())),
        ("school", format!("{}", hina.school())),
        ("club", format!("{}", hina.club())),
        ("armor", format!("{}", hina.armor())),
        (
            "weapon",
            format!(
                "[{}] - {} : {}",
                hina.weapon.name,
                hina.weapon_type(),
                hina.bullet_type()
            ),
        ),
    ];
    let max = segments.iter().map(|(n, _)| n.len()).max().unwrap();
    segments.map(|(name, details)| println!("{}{name}: {}", " ".repeat(max - name.len()), details));

    println!();

    // We can also fetch a random student.
    let random_student = (blue_archive::fetch_random_student(&Language::English).await?).unwrap();
    // I wonder who it will be this time?
    println!(
        "The random student of this second is: {}!",
        random_student.full_name_last()
    );
    // All of these are available in the StudentFetcher as well.
    // Though, check the student_fetcher example to see that.
    Ok(())
}
