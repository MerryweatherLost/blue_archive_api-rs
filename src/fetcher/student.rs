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
    /// Creates a new **[`StudentFetcher`]** by fetching **[`Student`]** data.
    /// Has a chance to fail as it attempts to fetch all students.
    pub async fn new(
        language: impl std::borrow::Borrow<Language>,
    ) -> Result<Self, BlueArchiveError> {
        Ok(Self {
            students: crate::fetch_all_students(language).await?,
        })
    }

    /**
    Gets a **[`Student`]** by a `name` from a set of names.

    ## Different Methods
    - Searching with an associated tag, such as **`Iori (Swimsuit)`**
        - It is recommended to use the last name and an associated tag if you are looking for a **[`Student`]** with a different appearance.
    - Searching via. the **last name**.
    - Searching via. the **first name**.
    - Searching via. the **first name and last name together**, and vise versa (e.g. Ichinose Asuna/Asuna Ichinose).

    # Examples
    ```
    use anyhow::Result;
    use blue_archive::{Language, StudentFetcher};

    #[tokio::main]
    async fn main() -> Result<()> {
        // Obtaining our fetcher struct...
        let fetcher = StudentFetcher::new(Language::English).await?;
        // We will use the first name and last name for this example.
        let asuna = fetcher.get_student_by_name("Asuna Ichinose").unwrap();
        println!("{asuna}");
        Ok(())
    }
    ```
    */
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
