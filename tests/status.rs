use std::cmp::Ordering;

/// Check if the API is `200` **OK.**
#[tokio::test]
async fn ok_status() {
    let status_code = match blue_archive::fetch_status().await {
        Ok(status) => Some(status.code),
        Err(_) => None,
    };
    assert_eq!(status_code.cmp(&Some(200)), Ordering::Equal)
}
