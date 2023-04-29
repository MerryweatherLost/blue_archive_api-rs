use crate::{filter::student::StudentFilterOptions, types::Student, BlueArchiveError, Language};

use anyhow::Result;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct StudentFetcher {
    pub students: Vec<Student>,
}
impl StudentFetcher {
    /// Creates a new **[`StudentFetcher`]** by fetching [`Student`] data and hopefully returning the result.
    pub async fn new(language: &Language) -> Result<Self, BlueArchiveError> {
        Ok(Self {
            students: crate::fetch_all_students(language).await?,
        })
    }

    /// Gets a **[`Student`]** by a `name` from a set of pattern matching.
    pub fn get_student_by_name(&self, name: impl Into<String>) -> Option<&Student> {
        let name: String = name.into();
        self.students.iter().find(|student| {
            [
                &student.first_name(),
                &student.last_name(),
                &student.full_name_with_last(),
                &student.full_name_with_first(),
            ]
            .into_iter()
            .any(|x| x.to_lowercase() == name.to_lowercase())
        })
    }

    /// Attempts to get a random **[`Student`]**.
    ///
    /// If the [`Vec`] is empty, then it will return [`None`].
    pub fn get_random_student(&self) -> Option<&Student> {
        self.students.choose(&mut rand::thread_rng())
    }

    pub fn filter(&self) -> StudentFilterOptions {
        StudentFilterOptions::new(&self.students)
    }
}
