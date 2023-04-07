use anyhow::Result;
use blue_archive::Region;

#[tokio::main]
async fn main() -> Result<()> {
    // Fetching a student from a region should be associated with the actual name in that region.
    let カホ = blue_archive::fetch_student_by_name("カホ", Some(Region::Japan)).await?;
    println!("{カホ}"); // (#10065 :: カホ :: Age: 17 :: School:Hyakkiyako)

    // Although you may use this and it would still work, though the name will not return as Japanese.
    let asuna = blue_archive::fetch_student_by_name("Asuna", Some(Region::Japan)).await?;
    println!("{asuna}\nProfile: {}", asuna.profile());
    // (#16001 :: Asuna :: Age: 17 :: School:Millennium)
    // Profile: ミレニアムサイエンススクール所属、秘密組織「C&C」のエージェント。
    Ok(())
}
