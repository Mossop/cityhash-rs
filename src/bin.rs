extern crate cityhash;

#[macro_use]
extern crate clap;
use clap::App;

use cityhash::cityhash_1;
use cityhash::cityhash_1_1_1;

enum Encoding {
    Utf8,
    Utf16,
    Unicode,
}

enum Endianness {
    BigEndian,
    LittleEndian,
    Native,
}

fn char_to_unicode(ch: char, endian: &Endianness) -> Vec<u8> {
    match endian {
        Endianness::Native => (ch as u32).to_ne_bytes().to_vec(),
        Endianness::BigEndian => (ch as u32).to_be_bytes().to_vec(),
        Endianness::LittleEndian => (ch as u32).to_le_bytes().to_vec(),
    }
}

fn char_to_utf16(ch: char, endian: &Endianness) -> Vec<u8> {
    let mut buffer = [0; 2];
    ch.encode_utf16(&mut buffer)
        .iter()
        .map(|u: &u16| match endian {
            Endianness::Native => u.to_ne_bytes().to_vec(),
            Endianness::BigEndian => u.to_be_bytes().to_vec(),
            Endianness::LittleEndian => u.to_le_bytes().to_vec(),
        })
        .flatten()
        .collect()
}

fn char_to_utf8(ch: char) -> Vec<u8> {
    let mut buffer = [0; 4];
    ch.encode_utf8(&mut buffer).as_bytes().into()
}

fn hash(
    version: &str,
    bits: &str,
    encoding: &Encoding,
    endianness: &Endianness,
    string: &str,
) -> String {
    let bytes: Vec<u8> = string
        .chars()
        .map(|ch| match encoding {
            Encoding::Utf8 => char_to_utf8(ch),
            Encoding::Utf16 => char_to_utf16(ch, &endianness),
            Encoding::Unicode => char_to_unicode(ch, &endianness),
        })
        .flatten()
        .collect();

    match bits {
        "64" => format!(
            "{:X}",
            match version {
                "1" => cityhash_1::city_hash_64(&bytes),
                "1.1.1" => cityhash_1_1_1::city_hash_64(&bytes),
                _ => panic!("Unknown CityHash version."),
            }
        ),
        "128" => format!(
            "{:X}",
            match version {
                "1" => cityhash_1::city_hash_128(&bytes),
                "1.1.1" => cityhash_1_1_1::city_hash_128(&bytes),
                _ => panic!("Unknown CityHash version."),
            }
        ),
        _ => panic!("Unexpected number of hash bits requested."),
    }
}

pub fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .author(crate_authors!(" "))
        .get_matches();

    let version = matches.value_of("version").unwrap();
    let bits = matches.value_of("bits").unwrap();
    let (encoding, endianness) = match matches.value_of("encoding").unwrap() {
        "utf8" => (Encoding::Utf8, Endianness::Native),
        "utf16" => (Encoding::Utf16, Endianness::Native),
        "utf16_le" => (Encoding::Utf16, Endianness::LittleEndian),
        "utf16_be" => (Encoding::Utf16, Endianness::BigEndian),
        "unicode" => (Encoding::Unicode, Endianness::Native),
        "unicode_le" => (Encoding::Unicode, Endianness::LittleEndian),
        "unicode_be" => (Encoding::Unicode, Endianness::BigEndian),
        _ => panic!("Unexpected encoding."),
    };

    let strings: Vec<&str> = matches.values_of("string").unwrap().collect();
    let length = strings.len();
    for string in strings {
        let hash = hash(version, bits, &encoding, &endianness, &string);
        if length > 1 {
            println!("\"{}\" => {}", string, hash);
        } else {
            println!("{}", hash);
        }
    }
}
