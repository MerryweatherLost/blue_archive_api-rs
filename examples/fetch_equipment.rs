use blue_archive::{types::equipment::EquipmentCategory, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    for equipment in blue_archive::fetch_all_equipment(Language::English).await? {
        println!("{}", equipment.name)
    }

    let hats = blue_archive::fetch_equipment_by_category(Language::English, EquipmentCategory::Hat)
        .await?;

    for hat in &hats {
        println!("[{}] -> {}", hat.id, hat.name)
    }

    Ok(())
}
