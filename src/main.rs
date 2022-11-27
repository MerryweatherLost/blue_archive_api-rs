#[tokio::main]
async fn main() {
    match blue_archive::fetch_banners().await {
        Ok(banners) => {
            for banner in banners.ended.iter() {
                println!("{}, {}", banner.id, banner.started_at)
            }
        }
        Err(err) => println!("{}", err),
    }
}
