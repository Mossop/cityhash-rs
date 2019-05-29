//! Provides rust bindings for the [CityHash algorithm](https://github.com/google/cityhash).
//!
//! The functions exposed here use the latest CityHash version, currently v1.1.1.

pub mod cityhash_1;
pub mod cityhash_1_1_1;

pub use cityhash_1_1_1::{city_hash_128, city_hash_64};
