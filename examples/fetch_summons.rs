use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can fetch all summons.
    let summons = blue_archive::fetch_all_summons(Language::English).await?;
    // Let's iterate over all of them and print out some details...
    summons.iter().for_each(|summon| {
        println!(
            "{} : {} : Ammo Cost: {} : Weapon: {}, Armor: {}",
            summon.id,
            summon.name,
            summon.ammo_cost,
            summon.weapon_type(),
            summon.armor()
        )
    });
    Ok(())
}
