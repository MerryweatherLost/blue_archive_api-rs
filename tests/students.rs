#[tokio::test]
async fn fetch_asuna() {
    let name = match blue_archive::fetch_student_by_name("Asuna").await {
        Ok(student) => student.character.name,
        Err(_) => "INVALID".to_string(),
    };
    assert_eq!("Asuna", name)
}
