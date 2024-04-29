use blue_archive::Language;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "Total Currencies: [{}]",
        blue_archive::fetch_all_currencies(Language::English)
            .await?
            .len()
    );

    println!("Pyroxenes");
    println!("--------------------------");
    println!(
        "{:#?}",
        blue_archive::fetch_currency_by_name("Pyroxenes", Language::English)
            .await?
            .unwrap()
    );
    Ok(())
}
