use blue_archive::Role;

/// Check if **Asuna** can be acquired through means of a [`String`] query.
#[tokio::test]
async fn fetch_asuna() {
    assert!(blue_archive::fetch_student_by_name("Asuna").await.is_ok())
}

/// Check if **Koharu** can be found by their ID represented in the API, which is `10020`.
#[tokio::test]
async fn fetch_koharu_by_id() {
    assert!(blue_archive::fetch_student_by_id(10020).await.is_ok())
}

/// Fetches all students that have the Role of Attacker.
#[tokio::test]
async fn fetch_attacker_students() {
    assert!(blue_archive::fetch_students_by_role(Role::Attacker)
        .await
        .is_ok())
}

/// Fetches all students that have Shotguns.
#[tokio::test]
async fn fetch_shotgun_students() {
    assert!(
        blue_archive::fetch_students_by_weapon(blue_archive::Weapon::SG)
            .await
            .is_ok()
    )
}
