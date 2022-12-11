#[tokio::test]
async fn fetch_asuna() {
    let name = match blue_archive::fetch_student_by_name("Asuna").await {
        Ok(student) => student.character.name,
        Err(_) => "INVALID".to_string(),
    };
    assert_eq!("Asuna", name)
}

#[tokio::test]
async fn ok_status() {
    let status_code = match blue_archive::fetch_status().await {
        Ok(status) => status.code,
        Err(_) => 0,
    };
    assert_eq!(status_code.cmp(&200), std::cmp::Ordering::Equal)
}
