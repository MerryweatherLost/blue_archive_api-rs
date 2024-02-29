use blue_archive::Language;
use rand::Rng;

fn read_line_to_buffer(buffer: &mut String) -> Result<usize, std::io::Error> {
    std::io::stdin().read_line(buffer)
}

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let mut input_buffer = String::new();

    println!("Guessing Game");
    println!("---------------------------");
    println!("See if you can guess the characters based on certain properties.\n\n");

    let chosen = blue_archive::fetch_random_student(Language::English)
        .await?
        .unwrap();
    let mut changed_name = &chosen.name.clone();
    let tng = rand::thread_rng().gen_range(0..changed_name.len());

    println!("Who is this character's name? [{chosen}]: ");

    read_line_to_buffer(&mut input_buffer)?;

    Ok(())
}
