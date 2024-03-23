#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raid_data = blue_archive::fetch_raid_data(&blue_archive::Language::English).await?;
    println!(
        "\x1b[1;33m{: <10}{: <20}{: <20}{: <10}\x1b[0m",
        "ID", "Name", "Faction", "Bullet Type"
    );
    for raid in raid_data.raids {
        println!(
            "{: <10}{: <20}{: <20}{: <10}",
            raid.id.to_u32(),
            raid.name,
            match raid.faction {
                Some(faction) => faction.to_string(),
                None => "Unknown".into(),
            },
            raid.bullet_type
        );
    }
    Ok(())
}
