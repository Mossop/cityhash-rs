//! This module provides Rust bindings for [CityHash version 1.1.1](https://github.com/google/cityhash/tree/4726e30231a5740b977c1d4c05429d07396e4315).
extern crate libc;
use libc::{size_t, uint64_t};

#[repr(C)]
struct uint128 {
  pub first: uint64_t,
  pub second: uint64_t,
}

impl From<uint128> for u128 {
  fn from(pair: uint128) -> u128 {
    let first: u128 = pair.first.into();
    let second: u128 = pair.second.into();

    (second << 64) + first
  }
}

#[link(name = "cityhash_1_1_1")]
extern "C" {
  #[link_name = "\u{1}__Z16CityHash64_1_1_1PKcm"]
  fn CityHash64(buffer: *const u8, length: size_t) -> uint64_t;

  #[link_name = "\u{1}__Z17CityHash128_1_1_1PKcm"]
  fn CityHash128(buffer: *const u8, length: size_t) -> uint128;
}

/// Returns a 64-bit CityHash of the given slice of bytes.
/// ```
/// # fn main() {
/// use cityhash::cityhash_1_1_1::city_hash_64;
///
/// assert_eq!(city_hash_64(b"sentences"), 0xCB79_39E3_91E7_901E);
/// # }
pub fn city_hash_64(buffer: &[u8]) -> u64 {
  unsafe { CityHash64(buffer.as_ptr(), buffer.len() as size_t) }
}

/// Returns a 128-bit CityHash of the given slice of bytes.
///
/// ```
/// # fn main() {
/// use cityhash::cityhash_1_1_1::city_hash_128;
///
/// assert_eq!(city_hash_128(b"sentences"), 0x0C86_49F6_4B31_D024_0DCC_335E_F92D_3406);
/// # }
pub fn city_hash_128(buffer: &[u8]) -> u128 {
  unsafe { CityHash128(buffer.as_ptr(), buffer.len() as size_t).into() }
}
