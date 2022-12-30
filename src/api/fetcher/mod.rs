use anyhow::Result;

use crate::types::*;

// todo: finish other functions and caching.
/// Allows for caching of Blue Archive data, with a cost of memory and time, although functions and accessing of data will be more easier.
/// It is recommended if you'd prefer hot-loading all the data first, **rather than making multiple asynchronous queries**, e.g. for fetching all instances of
/// [`Student`], which is difficult to fetch due to API limitations.
///
/// * Note that this is a **Work In Progress**, not everything is implemented at this point in time.
pub struct BlueArchiveFetcher {
    pub students: Vec<Student>,
}
impl BlueArchiveFetcher {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            students: crate::fetch_all_students().await?,
        })
    }

    pub fn get_student_by_name<IntoString: Into<String>>(
        &self,
        name: IntoString,
    ) -> Option<Student> {
        let name: String = name.into();
        self.students
            .iter()
            .find(|student| student.character.name.to_lowercase() == name.to_lowercase())
            .cloned()
    }
}
