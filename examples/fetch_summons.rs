use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can fetch all summons.
    let summons = blue_archive::fetch_all_summons(Language::English).await?;
    // Let's iterate over all of them and print out some details...
    summons.iter().for_each(|summon| {
        println!(
            "{} : {} : ammo cost : {}",
            summon.id, summon.name, summon.ammo_cost
        )
    });
    Ok(())
}
