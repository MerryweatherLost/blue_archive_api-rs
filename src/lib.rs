//! blue_archive
//! TBD...
//!

pub mod api;
pub mod enums;
pub mod errors;
pub mod fetcher;
pub mod filter;
pub mod types;

pub use api::{student::*, summon::*};
pub use enums::Language;
pub use errors::BlueArchiveError;
pub use filter::student::StudentFilter;

pub const DATA_URI: &str = "https://raw.githubusercontent.com/lonqie/SchaleDB/main/data";
pub const IMAGE_DATA_URI: &str = "https://raw.githubusercontent.com/lonqie/SchaleDB/main/images";
