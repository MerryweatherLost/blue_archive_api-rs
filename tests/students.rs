use blue_archive::{Club, Position, Query, Role, School, Weapon};

/// Check if **Asuna** can be acquired through means of a [`String`] query.
#[tokio::test]
async fn fetch_asuna_by_name() {
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
async fn fetch_srt_front_by_query() {
    assert!(blue_archive::fetch_students_by_queries([
        Query::School(School::SRT),
        Query::Position(Position::Front)
    ])
    .await
    .is_ok())
}

/// Fetches for Trinity Students apart of the club Sisterhood.
/// This should end up not being empty.
#[tokio::test]
async fn fetch_trinity_sisterhood_by_filter() -> anyhow::Result<()> {
    assert!(
        !blue_archive::filter(&blue_archive::fetch_all_students(None).await?)
            .apply(School::Trinity)
            .apply(Club::Sisterhood)
            .finish_ref()
            .is_empty()
    );
    Ok(())
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
