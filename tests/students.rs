use blue_archive::{Position, Query, Role, School, Weapon};

/// Check if **Asuna** can be acquired through means of a [`String`] query.
#[tokio::test]
async fn fetch_asuna() {
    assert!(blue_archive::fetch_student_by_name("Asuna", None)
        .await
        .is_ok())
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
    assert!(blue_archive::fetch_students_by_weapon(Weapon::SG)
        .await
        .is_ok())
}

/// Fetches **SRT** units that have **Front** positioning.
#[tokio::test]
async fn fetch_srt_front() {
    assert!(blue_archive::fetch_students_by_queries([
        Query::School(School::SRT),
        Query::Position(Position::Front)
    ])
    .await
    .is_ok())
}

/// Fetches a random student.
#[tokio::test]
async fn fetch_random_student() {
    assert!(blue_archive::fetch_random_student(None).await.is_ok())
}

/// Fetches Koharu by her ID.
#[tokio::test]
async fn fetch_koharu_by_id() {
    assert!(blue_archive::fetch_student_by_id(10020).await.is_ok())
}
