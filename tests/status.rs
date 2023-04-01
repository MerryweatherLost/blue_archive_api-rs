use std::cmp::Ordering;

/// Check if the API is `200` **OK.**
#[tokio::test]
async fn ok_status() {
    let status_code = match blue_archive::fetch_status().await {
        Ok(status) => status.code,
        Err(_) => 0,
    };
    assert_eq!(status_code.cmp(&200), Ordering::Equal)
}
