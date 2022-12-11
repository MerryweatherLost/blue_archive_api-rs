/// Check if **Asuna** can be acquired through means of a [`String`] query.
#[tokio::test]
async fn fetch_asuna() {
    let name = match blue_archive::fetch_student_by_name("Asuna").await {
        Ok(student) => student.character.name,
        Err(_) => "INVALID".to_string(),
    };
    assert_eq!("Asuna", name)
}

/// Check if **Koharu** can be found by their ID represented in the API, which is `10020`.
#[tokio::test]
async fn fetch_koharu_by_id() {
    let student_name = match blue_archive::fetch_student_by_id(10020).await {
        Ok(koharu) => koharu.character.name,
        Err(_) => "N/A".to_string(),
    };
    assert_eq!(student_name, "Koharu".to_string())
}

/// Fetches all students that have shotguns and checks if it is not empty.
#[tokio::test]
async fn fetch_shotgun_students() {
    let shotgun_students =
        match blue_archive::fetch_students_by_weapon(blue_archive::Weapon::SG).await {
            Ok(students) => students,
            Err(_) => {
                vec![]
            }
        };
    assert!(!shotgun_students.is_empty())
}
