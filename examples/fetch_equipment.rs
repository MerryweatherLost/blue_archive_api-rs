use blue_archive::{types::equipment::EquipmentCategory, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{:<60} Category", "Name");
    for equipment in blue_archive::fetch_all_equipment(Language::English).await? {
        println!("{:<60} {:?}", equipment.name, equipment.category);
    }

    let hats = blue_archive::fetch_equipment_by_category(Language::English, EquipmentCategory::Hat)
        .await?;

    println!("\nHats");
    for hat in &hats {
        println!("[{}] -> {}", hat.id, hat.name)
    }

    Ok(())
}
