use self::students::StudentQuery;

pub mod students;

pub use students::Query;

pub enum EquipmentIDOrName {
    ID(u32),
    Name(String),
}

pub enum Endpoints {
    Status,
    /**
        Takes a [`StudentQuery`].
    */
    Character(StudentQuery),
    /**
        Takes a `ID`: ([`u32`]) or a `String`: [`String`], to use for querying data about equipment.
    */
    Equipment(EquipmentIDOrName),
    Stage,  // todo
    Raid,   // todo
    Banner, // todo
}
