use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let status = blue_archive::fetch_status().await?;
    println!("{:?}", status.uptime());
    Ok(())
}
