use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    for equipment in blue_archive::fetch_all_equipment(Language::English).await? {
        println!("{}", equipment.name)
    }
    Ok(())
}
