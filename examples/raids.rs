#[tokio::main]
async fn main() {
    if let Ok(raids) = blue_archive::fetch_raids().await {
        for raid in raids.ended {
            if let Ok(start_time) = raid.start_time() {
                println!("Boss: {}, {}", raid.boss_name, start_time)
            }
        }
    }
}
