use blue_archive::{types::Student, Language};
use rand::Rng;

fn read_line_to_buffer(buffer: &mut String) -> Result<usize, std::io::Error> {
    std::io::stdin().read_line(buffer)
}

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let mut input_buffer = String::new();

    println!("Guessing Game (it's really bad)");
    println!("---------------------------");
    println!("See if you can guess the characters based on certain properties.\n\n");

    let chosen = blue_archive::fetch_random_student(Language::English)
        .await?
        .unwrap();
    let mut changed_name = chosen.name.clone();

    let index = rand::thread_rng().gen_range(0..chosen.name.len());
    changed_name.remove(index);
    changed_name.insert(index, '_');

    println!("Who is this character's name? [{changed_name}]: ");

    read_line_to_buffer(&mut input_buffer)?;

    println!("Your input: {}", input_buffer.trim());

    if input_buffer.trim().to_lowercase() == chosen.name.to_lowercase() {
        println!("Yay! You got it right!")
    } else {
        println!(
            "Aw... you got it wrong... it was actually: {}.",
            chosen.name
        )
    }

    Ok(())
}
