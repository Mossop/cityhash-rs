//! Provides rust bindings for the [CityHash algorithm](https://github.com/google/cityhash).
//!
//! This crate contains multiple versions of the CityHash algorithm since the
//! hash returned varies depending on the version used. Each version is in a
//! versioned sub-module with `.` characters replaced by `_`.
//!
//! If you want what you're building to have a stable hashing function then use
//! the functions from one of the versioned sub-modules. Otherwise the top-level
//! module will always expose the functions from the latest included version.
//!
//! Currently only the 64-bit and 128-bit hashing functions with no seeds are
//! exposed, but it would be trivial to add them should someone feel like it!

pub mod cityhash_1;
pub mod cityhash_1_1_1;

pub use cityhash_1_1_1::{city_hash_128, city_hash_64};
