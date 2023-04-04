use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let equipment = blue_archive::fetch_equipment_by_id(3000).await?;
    println!("{equipment:#?}");
    Ok(())
}
