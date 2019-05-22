extern crate cc;

use std::env;

fn main() {
    env::set_current_dir("cityhash").unwrap();
    cc::Build::new()
        .cpp(true)
        .file("city.cc")
        .compile("cityhash");
}
