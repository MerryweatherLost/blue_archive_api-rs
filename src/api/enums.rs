use crate::{Armor, Damage, Position, Role, School, SquadType, Weapon};

/// An `enum` that represents querying the API through two choices, [`CharacterNameOrQuery::Name`] (searching the student by name) or [`CharacterNameOrQuery::Query`] (through a specified [`Vec`] of [`StudentQuery`]'s).
pub enum CharacterNameOrQuery {
    Name(String),
    Query(Vec<StudentQuery>),
}

pub enum EquipmentIDOrName {
    ID(u32),
    Name(String),
}

pub enum Endpoints {
    Status,
    /**
        Takes a [`Option<CharNameOrQuery>`], which is used to pass in a
        `Name`: ([`String`]), or a `Query`: ([`StudentQuery`]) for advanced queries.

        If [`None`], then it will get a result of all partial students.
    */
    Character(Option<CharacterNameOrQuery>),
    /**
        Takes a `ID`: ([`u32`]) or a `String`: [`String`], to use for querying data about equipment.
    */
    Equipment(EquipmentIDOrName),
    Stage,  // todo
    Raid,   // todo
    Banner, // todo
}

/**
    A `enum` that maps queries with given data in each of them.
*/

pub enum StudentQuery {
    Role(Role),
    SquadType(SquadType),
    School(School),
    Position(Position),
    Weapon(Weapon),
    Damage(Damage),
    Armor(Armor),
}

impl std::fmt::Display for StudentQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Role(q) => write!(f, "role={}", q),
            Self::SquadType(q) => write!(f, "type={}", q),
            Self::School(q) => write!(f, "school={}", q),
            Self::Position(q) => write!(f, "position={}", q),
            Self::Weapon(q) => write!(f, "weapon={}", q),
            Self::Damage(q) => write!(f, "damage={}", q),
            Self::Armor(q) => write!(f, "heavy%20armor={}", q),
        }
    }
}
