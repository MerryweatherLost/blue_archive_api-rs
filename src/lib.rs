//! blue_archive-rs
//! ---------------
//! A Blue Archive API wrapper written in Rust.
//!
//! It mainly holds information from the **Global** Version.
//! The API primarily focuses on drop rates of each stages on Blue Archive.
//!
//! The Rest API that is used is from [**torikushiii**](https://github.com/torikushiii).
//!
//! You can find it here: **<https://github.com/torikushiii/BlueArchiveAPI>**
//!

pub mod api;
pub mod enums;
pub mod types;

pub use enums::*;

pub use api::*;
pub use fetcher::BlueArchiveFetcher;
