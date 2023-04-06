//! Contains the [`BlueArchiveFetcher`] and its methods and implementations.

use anyhow::Result;
use rand::seq::SliceRandom;

use crate::{filter::StudentFilterOptions, types::*, BlueArchiveError, Region};

/// Allows for caching of Blue Archive data, with a cost of memory and time, although functions and accessing of data will be more easier.
/// It is recommended if you'd prefer hot-loading all the data first, **rather than making multiple asynchronous queries**, e.g. for fetching all instances of
/// [`Student`], which is difficult to fetch due to API limitations.

#[derive(Debug, Default, Clone)]
pub struct BlueArchiveFetcher {
    pub students: Vec<Student>,
    pub region: Region,
}
impl BlueArchiveFetcher {
    /// Refreshes the cache with new data, should be used rarely.
    pub async fn refresh(&mut self) -> Result<(), BlueArchiveError> {
        self.students = crate::fetch_all_students(Some(self.region.clone())).await?;
        Ok(())
    }

    /// Allows for the creation of a new fetcher, though can fail if there are API issues, such as it being down.
    ///
    /// [`Region`] will default to [`Region::Global`] if [`None`].
    pub async fn new(region: Option<Region>) -> Result<Self> {
        Ok(Self {
            students: crate::fetch_all_students(region.clone()).await?,
            region: region.unwrap_or_default(),
        })
    }

    /// Allows you to search for a [`Student`] by their name, if they exist.
    pub fn get_student_by_name<IS: Into<String>>(&self, name: IS) -> Option<Student> {
        let name: String = name.into();
        self.students
            .iter()
            .find(|student| student.character.name.to_lowercase() == name.to_lowercase())
            .cloned()
    }

    /// Allows you to search for a [`Student`] by their id, if they exist.
    pub fn get_student_by_id(&self, id: u32) -> Option<Student> {
        self.students
            .iter()
            .find(|student| student.id == id)
            .cloned()
    }

    /// Fetches a random [`Student`], could be [`None`] if the `students` are empty.
    pub fn fetch_random_student(&self) -> Option<Student> {
        self.students.choose(&mut rand::thread_rng()).cloned()
    }

    /// Returns a [`StudentFilterOptions`], which allows for chaining of items that implement [`StudentFilter`].
    pub fn filter(&'_ self) -> StudentFilterOptions<'_> {
        StudentFilterOptions::new(&self.students)
    }
}
