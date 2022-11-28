use blue_archive::{self, types::Student};
use tokio;

#[tokio::test]
async fn fetch_asuna() {
    assert_eq!(
        "Asuna",
        blue_archive::fetch_student_by_name("Asuna")
            .await
            .unwrap()
            .character
            .name
    )
}
