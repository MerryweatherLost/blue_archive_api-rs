//! All [`StudentFilter`] implementations, including its own trait are in this module.

use crate::{
    enums::{Armor, BulletType, Club, Position, School, Squad, TacticalRole, WeaponType},
    types::{Age, Released, Student, ID},
};

/// Used to filter **[`Students`][`Student`]**.
pub trait StudentFilter {
    /// Filters a borrowed slice of [`Student`], and returns a **[`Vec<Student>`]**.
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student>;
}

impl StudentFilter for Age {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.age() == self)
            .collect()
    }
}

impl StudentFilter for Released {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.released == self)
            .collect()
    }
}

impl StudentFilter for ID {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.id == self)
            .collect()
    }
}

impl StudentFilter for School {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.school() == self)
            .collect()
    }
}

impl StudentFilter for TacticalRole {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.tactical_role() == self)
            .collect()
    }
}

impl StudentFilter for Squad {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.squad() == self)
            .collect()
    }
}

impl StudentFilter for Armor {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.armor() == self)
            .collect()
    }
}

impl StudentFilter for Position {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.position() == self)
            .collect()
    }
}

impl StudentFilter for BulletType {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.bullet_type() == self)
            .collect()
    }
}

impl StudentFilter for Club {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.club() == self)
            .collect()
    }
}

impl StudentFilter for WeaponType {
    fn filter<'a>(&self, students: &'a [Student]) -> Vec<&'a Student> {
        students
            .iter()
            .filter(|student| &student.weapon_type() == self)
            .collect()
    }
}

/// Provides the options to apply any **[`StudentFilter`]** on a **[`Vec<Student>`]**.
/// It is recommended to use the `new` function, or create it from **[the crate filter](`crate::filter`)**, or in **[`StudentFetcher`](`crate::StudentFetcher`)**.
#[derive(Debug)]
pub struct StudentFilterOptions<'s> {
    pub filtered_students: Option<Vec<&'s Student>>,
    slice: &'s [Student],
}

impl<'s> StudentFilterOptions<'s> {
    pub fn new(students: &'s Vec<Student>) -> Self {
        Self {
            filtered_students: None,
            slice: students,
        }
    }

    fn apply_filter(&mut self, student_filter: impl StudentFilter) {
        let mut applied_filter = student_filter.filter(self.slice);
        match &self.filtered_students {
            Some(filtered) => {
                applied_filter.retain(|student| filtered.contains(student));
                self.filtered_students = Some(applied_filter);
            }
            None => self.filtered_students = Some(applied_filter),
        }
    }

    /**
    Applies a filter, and returns [itself][`StudentFilterOptions`], allowing for direct chaining.

    The filter works by first filtering based on the [`StudentFilter`] implementation of the type,
    then matches with already existing **[`Students`][`Student`]** that were already filtered. It then finally retains the **[`Students`][`Student`]**
    within the filter with the previously filtered **[`Students`][`Student`]** in the [`filtered_students`](StudentFilterOptions::filtered_students) field.

    # Examples
    ```
    use anyhow::Result;

    use blue_archive::{School, Position}; // <- These enums implement StudentFilter.
    use blue_archive::Language;

    #[tokio::main]
    async fn main() -> Result<()> {
        let students = blue_archive::fetch_all_students(Language::English).await?;
        // We use a specific builder-like pattern to chain the applications of filters, and then finish them.
        // The first filter has precedence over the next.
        blue_archive::filter(&students)
            .apply(School::Abydos) // <- All students from Abydos will be filtered.
            .apply(Position::Front) // <- Then, all Front Position students (from Abydos due to precedence) will be filtered.
            .finish(); // <- Finally, we finish with a Vec of the Students as read-only values. (Vec<&Student>).
        Ok(())
    }
    ```
    */
    pub fn apply(mut self, student_filter: impl StudentFilter) -> Self {
        self.apply_filter(student_filter);
        self
    }
    /// Finishes the filtering process.
    /// If there is no filter applied using `apply`, then it will return an empty [`Vec`].
    pub fn finish(self) -> Vec<&'s Student> {
        self.filtered_students
            .unwrap_or_default()
            .into_iter()
            .collect()
    }
}
