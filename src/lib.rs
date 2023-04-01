//! blue_archive-rs
//! ---------------
//! A Blue Archive API wrapper written in Rust.
//!
//! **Consider this crate to be at an unfinished stage, though queries work properly, there is some work to be done with specific structures.**
//!
//! I'm also a beginner when it comes to this language, so a lot of stuff may be in bad practice! You can pitch in to flag issues or provide assistance [**here.**](https://github.com/MerryweatherLost/blue_archive_api-rs)
//!
//! It mainly holds information from the **Global** Version.
//! The API primarily focuses on drop rates of each stages on Blue Archive.
//!
//! The Rest API that is used is from [**torikushiii**](https://github.com/torikushiii).
//!
//! You can find it here: **<https://github.com/torikushiii/BlueArchiveAPI>**
//!
//! ### Feature Flags
//!
//! | Feature                      | Description                                                                                                                                                             | Default |
//! |:-----------------------------|:------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:--------|
//! | `query`                      | Allows for the usage of extra functions to fetch singular queries.                                                                                                      | YES     |

pub mod api;
pub mod enums;
pub mod types;

pub use enums::{Armor, Club, Damage, Position, Role, School, SquadType, Weapon};

pub use api::*;
pub use fetcher::BlueArchiveFetcher;

pub use api::enums::Query;
pub use errors::BlueArchiveError;

/// The API uri that is used for all the endpoints & queries.
pub const API_URI: &str = "https://api.ennead.cc/buruaka";
