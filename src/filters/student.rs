use crate::{enums::School, types::*};

pub trait StudentFilter {
    fn filter(self, students: &[Student]) -> Vec<&Student>;
}

impl StudentFilter for Age {
    fn filter(self, students: &[Student]) -> Vec<&Student> {
        students
            .iter()
            .filter(|student| self == student.age())
            .collect()
    }
}

// todo: resolve the complication in implementing a Released filter.
impl StudentFilter for Released {
    fn filter(self, students: &[Student]) -> Vec<&Student> {
        todo!()
    }
}

impl StudentFilter for StudentID {
    fn filter(self, students: &[Student]) -> Vec<&Student> {
        students
            .iter()
            .filter(|student| student.id() == self)
            .collect()
    }
}

impl StudentFilter for School {
    fn filter(self, students: &[Student]) -> Vec<&Student> {
        students
            .iter()
            .filter(|student| student.school() == self)
            .collect()
    }
}

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

    /// Applies a filter, and returns itself allowing for direct chaining.
    pub fn apply(mut self, student_filter: impl StudentFilter) -> Self {
        let mut applied_filter = student_filter.filter(self.slice);
        match self.filtered_students {
            Some(filtered) => {
                applied_filter.retain(|student| filtered.contains(student));
                self.filtered_students = Some(applied_filter);
            }
            None => self.filtered_students = Some(applied_filter),
        }
        self
    }

    /// Finishes the filtering and clones the values.
    ///
    /// If there is no filter applied using `apply`, then it will return an empty [Vec].
    pub fn finish(self) -> Vec<&'s Student> {
        self.filtered_students
            .unwrap_or_default()
            .into_iter()
            .collect()
    }
}
