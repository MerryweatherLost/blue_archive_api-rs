//! Contains the enums used in the API internally.

use self::students::StudentQuery;

pub mod students;

pub use students::Query;

/// An enum that matches either a `ID`: ([`u32`]) or a `Name`: ([`String`]).
#[derive(Debug)]
pub(crate) enum IDOrName {
    ID(u32),
    Name(String),
}

/// All of the API endpoints represented by enum values.
#[derive(Debug)]
pub(crate) enum Endpoints {
    Status,
    /**
        Takes a [`StudentQuery`].
    */
    Character(StudentQuery),
    /**
        Takes a `ID`: ([`u32`]) or a `String`: [`String`], to use for querying data about equipment.
    */
    Equipment(IDOrName),
    /// Blue Archive stages.
    ///
    /// <- underscored for now due to this endpoint not being documented (or I'm just not aware of it being done).
    _Stage,
    /// Related to raids in Blue Archive.
    Raid,
    /// Blue Archive banners.
    Banner,
}
