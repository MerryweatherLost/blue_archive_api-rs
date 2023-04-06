use crate::{enums::StudentFilter, types::Student};

#[derive(Debug)]
pub struct StudentFilterOptions<'student> {
    /// The [`Vec`] that will contain the filtered students. If [`None`], then a filter did not happen.
    pub filtered_students: Option<Vec<&'student Student>>,
    /// The known slice.
    known_slice: &'student [Student],
}
impl<'student> StudentFilterOptions<'student> {
    /// Creates a new filter options `struct`.
    pub fn new(students: &'student Vec<Student>) -> Self {
        Self {
            filtered_students: None,
            known_slice: students,
        }
    }
    /// Applies a filter, and returns itself, allowing for direct chaining.
    pub fn apply<F: StudentFilter>(mut self, student_filter: F) -> Self {
        match self.filtered_students {
            Some(current_filtered) => {
                let mut new_filtered = student_filter.filter(self.known_slice);
                new_filtered.retain(|student| current_filtered.contains(student));
                self.filtered_students = Some(new_filtered);
            }
            None => self.filtered_students = Some(student_filter.filter(self.known_slice)),
        }
        self
    }

    /// Finishes the filtering, and clones the values.
    pub fn finish(self) -> Vec<Student> {
        self.filtered_students
            .unwrap_or(vec![])
            .into_iter()
            .cloned()
            .collect()
    }

    /// Finishes the filtering, and returns the reference of the values.
    pub fn finish_ref(self) -> Vec<&'student Student> {
        self.filtered_students.unwrap_or(vec![])
    }
}
