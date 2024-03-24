use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let enemies = blue_archive::fetch_all_enemies(Language::English).await?;
    println!("{:<30} {:<30}", "Name", "Weapon");
    // division just to not flood your console.
    for i in 0..enemies.len() / 50 {
        let enemy = enemies.get(i).unwrap();
        println!("{:<30} {:<30}", enemy.name, enemy.weapon_type());
    }
    Ok(())
}
