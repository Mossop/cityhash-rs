//! This module provides Rust bindings for [CityHash version 1.1.1](https://github.com/google/cityhash/tree/4726e30231a5740b977c1d4c05429d07396e4315).

mod ffi {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/cityhash_1_1_1.rs"));

    impl From<uint128> for u128 {
        fn from(pair: uint128) -> u128 {
            let first: u128 = pair.first.into();
            let second: u128 = pair.second.into();

            (second << 64) + first
        }
    }
}

/// Returns a 64-bit CityHash of the given slice of bytes.
/// ```
/// # fn main() {
/// use cityhash::cityhash_1_1_1::city_hash_64;
///
/// assert_eq!(city_hash_64(b"sentences"), 0xCB79_39E3_91E7_901E);
/// # }
pub fn city_hash_64(buffer: &[u8]) -> u64 {
    unsafe { ffi::CityHash64_1_1_1(buffer.as_ptr() as *const _, buffer.len() as usize) }
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
    unsafe { ffi::CityHash128_1_1_1(buffer.as_ptr() as *const _, buffer.len() as usize).into() }
}
