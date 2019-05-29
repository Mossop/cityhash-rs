//! This module provides [CityHash version 1](https://github.com/google/cityhash/tree/6efae233528bbec2f057d2d1aa78f10d13fa73ea)
//! which returns different results than more modern versions.
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

#[link(name = "cityhash_1")]
extern "C" {
    #[link_name = "\u{1}__Z12CityHash64_1PKcm"]
    fn CityHash64(buffer: *const u8, length: size_t) -> uint64_t;

    #[link_name = "\u{1}__Z13CityHash128_1PKcm"]
    fn CityHash128(buffer: *const u8, length: size_t) -> uint128;
}

/// Returns a 64-bit CityHash of the given slice of bytes.
pub fn city_hash_64(buffer: &[u8]) -> u64 {
    unsafe { CityHash64(buffer.as_ptr(), buffer.len() as size_t) }
}

/// Returns a 128-bit CityHash of the given slice of bytes.
pub fn city_hash_128(buffer: &[u8]) -> u128 {
    unsafe { CityHash128(buffer.as_ptr(), buffer.len() as size_t).into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::EncodeUtf16;

    struct Utf16ByteIterator<'a> {
        base: EncodeUtf16<'a>,
        pending: Option<u8>,
    }

    impl<'a> Utf16ByteIterator<'a> {
        fn new(string: &str) -> Utf16ByteIterator {
            Utf16ByteIterator {
                base: string.encode_utf16(),
                pending: None,
            }
        }
    }

    impl<'a> Iterator for Utf16ByteIterator<'a> {
        type Item = u8;

        fn next(&mut self) -> Option<u8> {
            if let Some(b) = self.pending {
                self.pending = None;
                return Some(b);
            }

            match self.base.next() {
                Some(w) => {
                    let bytes = w.to_le_bytes();
                    self.pending = Some(bytes[1]);
                    Some(bytes[0])
                }
                None => None,
            }
        }
    }

    #[test]
    fn test_hash_64() {
        // From https://news.ycombinator.com/item?id=2434871
        assert_eq!(city_hash_64(b"I"), 0x641f_1cd0_505d_1ff9);
        assert_eq!(city_hash_64(b"do"), 0xe394_c683_1e9d_6e71);
        assert_eq!(city_hash_64(b"not"), 0x651c_1a4b_0b3f_2d88);
        assert_eq!(city_hash_64(b"know"), 0x9fc6_71af_2d05_1786);
        assert_eq!(city_hash_64(b"about"), 0x1fa3_8575_7f00_16ec);
        assert_eq!(city_hash_64(b"making"), 0xf240_7bd9_ce67_8f7f);
        assert_eq!(city_hash_64(b"awkward"), 0x73bd_225c_6b6b_8163);
        assert_eq!(city_hash_64(b"rhopalic"), 0xea78_562e_3577_7eb1);
        assert_eq!(city_hash_64(b"sentences"), 0x3c85_4a92_5e6e_4a9e);
        assert_eq!(city_hash_64(b"."), 0x84a0_4e9a_a8da_e9f5);

        // Known examples from Firefox.
        let bytes: Vec<u8> = Utf16ByteIterator::new(
            "/Users/dave/mozilla/source/trunk/obj-browser-opt-full/dist/Nightly.app/Contents/MacOS",
        )
        .collect();
        assert_eq!(city_hash_64(&bytes), 0x79dd_ed52_7a2b_a3af);

        let bytes: Vec<u8> =
            Utf16ByteIterator::new("/Applications/Firefox Nightly.app/Contents/MacOS").collect();
        assert_eq!(city_hash_64(&bytes), 0x3121_0a08_1f86_e80e);
    }

    #[test]
    fn test_hash_128() {
        let bytes: Vec<u8> = Utf16ByteIterator::new(
            "/Users/dave/mozilla/source/trunk/obj-browser-opt-full/dist/Nightly.app/Contents/MacOS",
        )
        .collect();
        assert_eq!(
            city_hash_128(&bytes),
            0x2572_72b5_e704_6575_1445_b259_c030_cf4b
        );

        assert_eq!(
            city_hash_128(b"sentences"),
            0xb439_3fa9_a2f4_e56c_3245_7785_c254_03d6
        );
    }
}
