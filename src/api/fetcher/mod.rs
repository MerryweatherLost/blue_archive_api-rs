//! Contains the [`BlueArchiveFetcher`] and its methods and implementations.

use anyhow::Result;
use rand::seq::SliceRandom;

use crate::{types::*, BlueArchiveError, Query};

/// Allows for caching of Blue Archive data, with a cost of memory and time, although functions and accessing of data will be more easier.
/// It is recommended if you'd prefer hot-loading all the data first, **rather than making multiple asynchronous queries**, e.g. for fetching all instances of
/// [`Student`], which is difficult to fetch due to API limitations.

pub struct BlueArchiveFetcher {
    pub students: Vec<Student>,
}
impl BlueArchiveFetcher {
    /// Refreshes the cache with new data, should be used rarely.
    pub async fn refresh(&mut self) -> Result<(), BlueArchiveError> {
        self.students = crate::fetch_all_students().await?;
        Ok(())
    }

    /// Allows for the creation of a new fetcher, though can fail if there are API issues, such as it being down.
    pub async fn new() -> Result<Self> {
        Ok(Self {
            students: crate::fetch_all_students().await?,
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

    /// Allows you to search for a specific set of [`Student`]'s based on a [`Vec`] of [`Query`]'s.
    pub fn get_students_by_queries<Q: Into<Vec<Query>>>(&self, queries: Q) -> Vec<Student> {
        let queries: Vec<Query> = queries.into();
        let mut filtered_students: Vec<Student> = vec![];
        for student in self.students.iter() {
            let mut checks: Vec<bool> = vec![];
            for query in queries.iter() {
                let result = match query {
                    Query::Role(role) => role == &student.role(),
                    Query::SquadType(squad) => squad == &student.squad_type(),
                    Query::School(school) => school == &student.school(),
                    Query::Position(position) => position == &student.position(),
                    Query::Weapon(weapon) => weapon == &student.weapon(),
                    Query::Damage(damage) => damage == &student.damage(),
                    Query::Armor(armor) => armor == &student.armor(),
                    Query::ID(id) => id == &student.id,
                    Query::Released(released) => released == &student.is_released,
                };
                checks.push(result)
            }
            if checks.iter().all(|&x| x) {
                filtered_students.push(student.clone())
            }
        }
        filtered_students
    }
}
