use blue_archive::School;

#[tokio::test]
async fn fetcher_test() -> anyhow::Result<()> {
    let student_fetcher = blue_archive::BlueArchiveFetcher::new(None).await?;

    let random_student = student_fetcher.fetch_random_student().is_some();
    let hina = student_fetcher.get_student_by_id(10004).is_some();
    let asuna = student_fetcher.get_student_by_name("Asuna").is_some();

    let trinity = !student_fetcher
        .filter()
        .apply(School::Trinity)
        .finish()
        .is_empty();

    println!("fetch random: {random_student} | fetch_student_by_id : {hina} | fetch_asuna_by_name: {asuna} | trinity_students: {trinity}");

    assert!(random_student && hina && asuna && trinity);
    Ok(())
}
