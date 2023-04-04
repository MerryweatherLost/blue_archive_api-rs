//! Enums related to querying students, and also contains a `StudentQuery`
//! and an associated builder pattern for internal usage.

use crate::{Armor, Damage, Position, Role, School, SquadType, Weapon};

/**
    A `enum` that maps queries with given data in each of them.
*/

#[derive(Debug)]
pub enum Query {
    Role(Role),
    SquadType(SquadType),
    School(School),
    Position(Position),
    Weapon(Weapon),
    Damage(Damage),
    Armor(Armor),
    ID(u32),
    Released(bool),
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Role(q) => write!(f, "role={}", q),
            Self::SquadType(q) => write!(f, "type={}", q),
            Self::School(q) => write!(f, "school={}", q),
            Self::Position(q) => write!(f, "position={}", q),
            Self::Weapon(q) => write!(f, "weapon={}", q),
            Self::Damage(q) => write!(f, "damage={}", q),
            Self::Armor(q) => write!(f, "heavy%20armor={}", q),
            Self::ID(id) => write!(f, "{id}?id=true"),
            Self::Released(released) => write!(f, "?released={released}"),
        }
    }
}

/// An `enum` that represents querying the API through three values,
/// - [`QueryKind::Name`] searching the student by name
/// - [`QueryKind::Single`] through a single [`Query`] (ID & released are applicable)
/// - [`QueryKind::Multiple`] through a specified [`Vec`] of [`Query`]
#[derive(Debug)]
pub(crate) enum QueryKind {
    Name(String),
    Single(Query),
    Multiple(Vec<Query>),
}

/// Builder pattern for a [`StudentQuery`], which allows for chaining of queries based on a [`QueryKind`].
///
/// **Meant for internal use.**
#[derive(Debug)]
pub(crate) struct StudentQueryBuilder {
    pub kind: Option<QueryKind>,
    pub query_string: String,
}

impl StudentQueryBuilder {
    pub fn new() -> Self {
        Self {
            kind: None,
            query_string: String::new(),
        }
    }

    /// Builds with "" as the String for the [`StudentQuery`].
    ///
    /// This is used in the case of fetching PartialStudents via the `character/` endpoint.
    pub fn build_empty(self) -> StudentQuery {
        StudentQuery("".to_string())
    }

    /// Allows for the building of a [`StudentQuery`] with the `name` of a student.
    pub fn build_with_student_name(mut self, name: String) -> StudentQuery {
        self.kind = Some(QueryKind::Name(name));
        self.build()
    }

    /// Allows for the building of a [`StudentQuery`] based on a **single** `query`.
    pub fn build_with_single(mut self, query: Query) -> StudentQuery {
        self.kind = Some(QueryKind::Single(query));
        self.build()
    }

    /// Allows for the building of a [`StudentQuery`] based on **multiple** `queries`: [`Vec<Query>`]
    pub fn build_with_multiple(mut self, queries: Vec<Query>) -> StudentQuery {
        self.kind = Some(QueryKind::Multiple(queries));
        self.build()
    }
    /// Build the StudentQuery, and depending if the query_kind is [`None`], it will result in a empty [`String`].
    fn build(mut self) -> StudentQuery {
        if let Some(query_kind) = self.kind {
            self.query_string = match query_kind {
                QueryKind::Single(query) => match query {
                    Query::ID(_) | Query::Released(_) => query.to_string(),
                    _ => format!("query?{}", query),
                },
                QueryKind::Multiple(queries) => format!(
                    "query?{}",
                    queries
                        .into_iter()
                        .map(|sq| sq.to_string())
                        .collect::<Vec<String>>()
                        .join("&")
                ),
                QueryKind::Name(name) => name,
            };
        }
        StudentQuery(self.query_string)
    }
}

/// A simple Query that wraps around a [`String`], and is to be used in fetching the response.
///
/// **Meant for internal use.**
#[derive(Debug)]
pub(crate) struct StudentQuery(String);

impl std::fmt::Display for StudentQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
