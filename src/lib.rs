//! blue_archive
//! ------------
//!
//! Rust API wrapper of lonqie's SchaleDB. For more information, go to these links:
//!
//! **Github & Owner:** <https://github.com/lonqie/SchaleDB> & <https://github.com/lonqie>
//!
//!
//! ## Information
//! Something I wanted to make in Rust was a simple api wrapper, picked something and the rest was history.
//! Much of it is still a work in progress.
//!
//! The former versions **`<= 2.*`** used a different API, and to migrate to the new one, a lot of changes needed to be done.
//!
//! **Crate Github:** <https://github.com/MerryweatherLost/blue_archive_api-rs>
//!
//!
//! ## Contributing
//! If you wish to help out, you can. I am a beginner in Rust, and I do not mind a few pointers.

pub mod api;
pub mod enums;
pub mod errors;
pub mod fetcher;
pub mod filter;
pub mod types;

pub use api::{student::*, summon::*};
pub use enums::{
    Armor, BulletType, Club, Language, Position, School, Squad, TacticalRole, WeaponType,
};
pub use errors::BlueArchiveError;
pub use fetcher::StudentFetcher;
pub use filter::student::StudentFilter;

pub const DATA_URI: &str = "https://raw.githubusercontent.com/lonqie/SchaleDB/main/data";
pub const IMAGE_DATA_URI: &str = "https://raw.githubusercontent.com/lonqie/SchaleDB/main/images";
