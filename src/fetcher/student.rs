//! Contains the **[`StudentFetcher`]** structure.

use crate::{filter::student::StudentFilterOptions, types::Student, BlueArchiveError, Language};

use anyhow::Result;
use rand::seq::SliceRandom;

/// Allows for caching of **[`Student`]** data with a cost of memory, although functions and accessing of data will be more easier.
/// It is recommended if you'd prefer hot-loading all the data first, **rather than making multiple asynchronous requests**.
#[derive(Debug, Clone)]
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

    /// Gets a **[`Student`]** by a `name` from a set of names.
    ///
    /// It is recommended to use the last name and an associated tag such as **`Iori (Swimsuit)`** of the **[`Student`]**.
    ///
    /// Until a proper system is figured out, perhaps with an associated [`Option`] of an `enum` that can discern a **[`Student`]** from their alternate with the same name.
    pub fn get_student_by_name(&self, name: impl Into<String>) -> Option<&Student> {
        let name: String = name.into();
        self.students.iter().find(|student| {
            [
                &student.name,
                &student.first_name(),
                &student.last_name(),
                &student.full_name_last(),
                &student.full_name_first(),
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

    /// Returns **[`StudentFilterOptions`]** to be used for filtering.
    pub fn filter(&self) -> StudentFilterOptions {
        StudentFilterOptions::new(&self.students)
    }
}
