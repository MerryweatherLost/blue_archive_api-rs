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
    - Searching via. the **first name and last name together**, and vise versa (e.g. Asuna Ichinose/Ichinose Asuna).

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
        let mut matched_student = None;

        for student in &self.students {
            let lowercased = name.to_lowercase();
            let maybe_student = (lowercased == student.name.to_lowercase()
                || lowercased == student.first_name.to_lowercase()
                || lowercased == student.last_name.to_lowercase()
                || lowercased == student.full_name_last().to_lowercase()
                || lowercased == student.full_name_first().to_lowercase())
            .then_some(student);
            if let Some(student) = maybe_student {
                matched_student = Some(student);
                break;
            }
        }

        matched_student
    }

    /// Attempts to get a random **[`Student`]**.
    ///
    /// If the [`Vec`] is empty, then it will return [`None`].
    pub fn get_random_student(&self) -> Option<&Student> {
        self.students.choose(&mut rand::thread_rng())
    }

    /// Attempts to get a random amount of **[`Students`][`Student`]** depending on the specified **`amount`**.
    ///
    /// Depending on the **`amount`** inserted, if it exceeds the total length of **[`Students`][`Student`]** from the data, it will just return everything.
    pub fn get_random_students(&self, amount: usize) -> Vec<&Student> {
        self.students
            .choose_multiple(&mut rand::thread_rng(), amount)
            .collect()
    }

    /// Returns **[`StudentFilterOptions`]** to be used for filtering.
    pub fn filter(&self) -> StudentFilterOptions {
        StudentFilterOptions::new(&self.students)
    }
}
