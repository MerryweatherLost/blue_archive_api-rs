#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raid_data = blue_archive::fetch_raid_data(&blue_archive::Language::English).await?;
    for raid in raid_data.raids {
        println!("({}, Armor Type: {})", raid.id, raid.armor())
    }
    Ok(())
}
