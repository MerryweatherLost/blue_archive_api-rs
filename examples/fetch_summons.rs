use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let summons = blue_archive::fetch_all_summons(&Language::Japanese).await?;
    for summon in &summons {
        println!("{} : ammo cost : {}", summon.id, summon.ammo_cost)
    }
    Ok(())
}
