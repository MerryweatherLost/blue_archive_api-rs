use anyhow::Result;
use blue_archive::Region;

#[tokio::main]
async fn main() -> Result<()> {
    let カホ = blue_archive::fetch_student_by_name("カホ", Some(Region::Japan)).await?;
    println!("{カホ}");
    Ok(())
}
