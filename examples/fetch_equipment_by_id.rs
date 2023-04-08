use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // You can fetch specific equipment through their ID.
    let equipment = blue_archive::fetch_equipment_by_id(3000).await?;
    println!("{equipment:#?}");
    Ok(())
}
